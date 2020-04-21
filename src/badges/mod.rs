#[derive(Debug, Deserialize)]
pub struct SvgBadgeInput {
    pub title: String,
    pub text: String,
    pub title_colour: Option<String>,
    pub text_colour: Option<String>,
    pub title_bg_colour: Option<String>,
    pub text_bg_colour: Option<String>,
    pub font_face: Option<minutiae::FontFace>,
    pub font_size: Option<minutiae::FontSize>,
    pub padding_horizontal: Option<i32>,
    pub padding_vertical: Option<i32>,
}

impl SvgBadgeInput {
    pub fn validate_n_populate(&mut self, factory: &Factory) -> Result<(), String> {
        self.validate_font(factory)
            .and(self.validate_colours())
            .and(self.validate_padding())
    }

    pub fn validate_padding(&mut self) -> Result<(), String> {
        if self.padding_horizontal.is_none() {
            self.padding_horizontal =
                Option::from(self.font_size.unwrap().parse::<i32>().unwrap() / 2)
        }

        if self.padding_vertical.is_none() {
            self.padding_vertical = Option::from(0)
        }

        Ok(())
    }

    pub fn validate_font(&mut self, f: &Factory) -> Result<(), String> {
        if self.font_face.is_none() {
            self.font_face = Option::from(f.default_font_face());
        }

        if self.font_size.is_none() {
            self.font_size = Option::from(f.default_font_size());
        }

        match f.supports_font(
            self.font_face.clone().unwrap(),
            self.font_size.clone().unwrap(),
        ) {
            true => Ok(()),
            false => Err("unsupported font".into()),
        }
    }

    pub fn validate_colours(&mut self) -> Result<(), String> {
        if self.title_colour.is_none() {
            self.title_colour = Option::from(String::from("#fff"));
        }

        if self.title_bg_colour.is_none() {
            self.title_bg_colour = Option::from(String::from("#000"));
        }

        if self.text_colour.is_none() {
            self.text_colour = Option::from(String::from("#000"));
        }

        if self.text_bg_colour.is_none() {
            self.text_bg_colour = Option::from(String::from("#fff"));
        }

        for s in vec![
            self.title_colour.clone(),
            self.title_bg_colour.clone(),
            self.text_colour.clone(),
            self.text_bg_colour.clone(),
        ] {
            let colour = s.clone().unwrap();

            if !colour.starts_with("#") {
                return Err(String::from(format!("invalid colour: {}", colour)));
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct FactoryOptions {
    pub render_dataset: minutiae::DataSet,
}

#[derive(Clone)]
pub struct Factory {
    opts: FactoryOptions,
}

impl Factory {
    pub fn new(opts: FactoryOptions) -> Factory {
        info!("building factory");

        Factory { opts }
    }

    pub fn default_font_face(&self) -> minutiae::FontFace {
        self.opts.render_dataset.config.font.faces[0].clone()
    }

    pub fn default_font_size(&self) -> minutiae::FontSize {
        self.opts.render_dataset.config.font.sizes[0].clone()
    }

    pub fn supports_font(&self, face: minutiae::FontFace, size: minutiae::FontSize) -> bool {
        self.opts.render_dataset.config.font.faces.contains(&face)
            && self.opts.render_dataset.config.font.sizes.contains(&size)
    }

    pub fn render_svg(&self, mut input: SvgBadgeInput) -> Result<String, String> {
        let r = input.validate_n_populate(self);

        if r.is_err() {
            return Err(r.err().unwrap());
        }

        let bbox = self.opts.render_dataset.bounding_box(
            &format!("{} - {}", input.title, input.text),
            minutiae::BoundingBoxRenderOptions {
                face: "Verdana".to_string(),
                size: "100".to_string(),
            },
        );

        match bbox {
            Some(v) => Ok(format!("{} - {} ({} x {})", input.title, input.text, v[0], v[1]).into()),
            None => Err("failed to render badge".into()),
        }
    }
}

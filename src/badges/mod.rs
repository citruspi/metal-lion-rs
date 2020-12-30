use glyph_bbox::dataset as GlyphDataSet;
use htmlescape as escape;
use liquid;
use rust_embed::RustEmbed;
use simple_icons;

#[derive(RustEmbed)]
#[folder = "assets/badges"]
struct Asset;

#[derive(Debug, Deserialize)]
pub struct SvgBadgeInput {
    pub title: String,
    pub text: Option<String>,
    pub title_colour: Option<String>,
    pub text_colour: Option<String>,
    pub title_bg_colour: Option<String>,
    pub text_bg_colour: Option<String>,
    pub font_face: Option<GlyphDataSet::FontFace>,
    pub font_size: Option<GlyphDataSet::FontSize>,
    pub padding_horizontal: Option<f64>,
    pub padding_vertical: Option<f64>,
    pub icon: Option<String>,
    pub icon_colour: Option<String>,
    pub icon_scale: Option<String>,
}

impl SvgBadgeInput {
    pub fn validate_n_populate(&mut self, factory: &Factory) -> Result<(), String> {
        self.validate_font(factory)
            .and(self.validate_colours())
            .and(self.validate_padding())
            .and(self.validate_icon())
            .and(self.sanitize_input())
    }

    pub fn sanitize_input(&mut self) -> Result<(), String> {
        self.title = escape::encode_minimal(&self.title);

        if self.text.is_some() {
            self.text = Some(escape::encode_minimal(&self.text.clone().unwrap()));
        }

        Ok(())
    }

    pub fn validate_icon(&mut self) -> Result<(), String> {
        if self.icon.is_none() {
            return Ok(());
        }

        if self.icon_scale.is_none() {
            self.icon_scale = Option::from(String::from("0.9"))
        }

        match simple_icons::get(&self.icon.clone().unwrap()) {
            Some(_v) => Ok(()),
            None => Err(String::from("invalid icon")),
        }
    }

    pub fn validate_padding(&mut self) -> Result<(), String> {
        if self.padding_horizontal.is_none() {
            self.padding_horizontal =
                Option::from(self.font_size.as_ref().unwrap().parse::<f64>().unwrap() / 2.0)
        }

        if self.padding_vertical.is_none() {
            self.padding_vertical =
                Option::from(self.font_size.as_ref().unwrap().parse::<f64>().unwrap() / 8.0)
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

        if self.icon_colour.is_none() {
            self.icon_colour = self.title_colour.clone();
        }

        for s in vec![
            self.title_colour.clone(),
            self.title_bg_colour.clone(),
            self.text_colour.clone(),
            self.text_bg_colour.clone(),
            self.icon_colour.clone(),
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
    pub render_dataset: GlyphDataSet::DataSet,
    pub host: String,
}

#[derive(Clone)]
pub struct Factory {
    opts: FactoryOptions,
    svg_template: String,
}

impl Factory {
    pub fn new(opts: FactoryOptions) -> Factory {
        info!("building factory");

        let svg_template: String =
            std::str::from_utf8(Asset::get("template.svg").unwrap().as_ref())
                .unwrap()
                .into();

        liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&svg_template.clone())
            .unwrap();

        Factory { opts, svg_template }
    }

    pub fn render_endpoint(&self) -> String {
        format!("{}/v1/badge.svg", self.opts.host)
    }

    pub fn font_faces(&self) -> GlyphDataSet::FontFaces {
        self.opts.render_dataset.config.font.faces.clone()
    }

    pub fn font_sizes(&self) -> GlyphDataSet::FontSizes {
        self.opts.render_dataset.config.font.sizes.clone()
    }

    pub fn default_font_face(&self) -> GlyphDataSet::FontFace {
        self.opts.render_dataset.config.font.faces[0].clone()
    }

    pub fn default_font_size(&self) -> GlyphDataSet::FontSize {
        self.opts.render_dataset.config.font.sizes[0].clone()
    }

    pub fn supports_font(
        &self,
        face: GlyphDataSet::FontFace,
        size: GlyphDataSet::FontSize,
    ) -> bool {
        self.opts.render_dataset.config.font.faces.contains(&face)
            && self.opts.render_dataset.config.font.sizes.contains(&size)
    }

    pub fn template(&self) -> liquid::Template {
        liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(&self.svg_template)
            .unwrap()
    }

    pub fn render_error_badge(&self, s: String) -> String {
        self.render_svg(SvgBadgeInput {
            title: "error".to_string(),
            text: Option::from(s),
            title_colour: None,
            text_colour: None,
            title_bg_colour: Option::from(String::from("#c0392b")),
            text_bg_colour: None,
            font_face: None,
            font_size: None,
            padding_horizontal: None,
            padding_vertical: None,
            icon: None,
            icon_colour: None,
            icon_scale: None,
        })
        .unwrap()
    }

    pub fn render_svg(&self, mut input: SvgBadgeInput) -> Result<String, String> {
        let r = input.validate_n_populate(self);

        if r.is_err() {
            return Err(r.err().unwrap());
        }

        let title_bbox = self.opts.render_dataset.bounding_box(
            &input.title,
            GlyphDataSet::BoundingBoxRenderOptions {
                face: input.font_face.clone().unwrap(),
                size: input.font_size.clone().unwrap(),
            },
        );

        let text_bbox = if input.text.is_some() {
            self.opts.render_dataset.bounding_box(
                &input.text.clone().unwrap(),
                GlyphDataSet::BoundingBoxRenderOptions {
                    face: input.font_face.clone().unwrap(),
                    size: input.font_size.clone().unwrap(),
                },
            )
        } else {
            None
        };

        if title_bbox.is_none() {
            return Err("failed to render badge".into());
        }

        let output = match (input.icon.is_some(), text_bbox.is_some()) {
            (true, true) => self.template().render(&liquid::object!({
                "title": input.title,
                "title_width": title_bbox.clone().unwrap()[0],
                "title_height": title_bbox.unwrap()[1],
                "text": input.text,
                "text_width": text_bbox.clone().unwrap()[0],
                "text_height": text_bbox.unwrap()[1],
                "font_face": input.font_face,
                "font_size": input.font_size,
                "title_colour": input.title_colour,
                "title_bg_colour": input.title_bg_colour,
                "text_colour": input.text_colour,
                "text_bg_colour": input.text_bg_colour,
                "padding_horizontal": input.padding_horizontal,
                "padding_vertical": input.padding_vertical,
                "icon": true,
                "icon_title": format!("{} icon", input.icon.clone().unwrap()),
                "icon_path": simple_icons::get(&input.icon.unwrap()).unwrap().path,
                "icon_colour": input.icon_colour,
                "icon_scale": input.icon_scale,
                "contains_text": true,
            })),
            (true, false) => self.template().render(&liquid::object!({
                "title": input.title,
                "title_width": title_bbox.clone().unwrap()[0],
                "title_height": title_bbox.unwrap()[1],
                "text": "",
                "contains_text": false,
                "text_width": 0,
                "text_height": 0,
                "font_face": input.font_face,
                "font_size": input.font_size,
                "title_colour": input.title_colour,
                "title_bg_colour": input.title_bg_colour,
                "text_colour": input.text_colour,
                "text_bg_colour": input.text_bg_colour,
                "padding_horizontal": input.padding_horizontal,
                "padding_vertical": input.padding_vertical,
                "icon": true,
                "icon_title": format!("{} icon", input.icon.clone().unwrap()),
                "icon_path": simple_icons::get(&input.icon.unwrap()).unwrap().path,
                "icon_colour": input.icon_colour,
                "icon_scale": input.icon_scale,
            })),
            (false, true) => self.template().render(&liquid::object!({
                "title": input.title,
                "title_width": title_bbox.clone().unwrap()[0],
                "title_height": title_bbox.unwrap()[1],
                "text": input.text,
                "text_width": text_bbox.clone().unwrap()[0],
                "text_height": text_bbox.unwrap()[1],
                "font_face": input.font_face,
                "font_size": input.font_size,
                "title_colour": input.title_colour,
                "title_bg_colour": input.title_bg_colour,
                "text_colour": input.text_colour,
                "text_bg_colour": input.text_bg_colour,
                "padding_horizontal": input.padding_horizontal,
                "padding_vertical": input.padding_vertical,
                "contains_text": true,
            })),
            (false, false) => self.template().render(&liquid::object!({
                "title": input.title,
                "title_width": title_bbox.clone().unwrap()[0],
                "title_height": title_bbox.unwrap()[1],
                "text": "",
                "contains_text": false,
                "text_width": 0,
                "text_height": 0,
                "font_face": input.font_face,
                "font_size": input.font_size,
                "title_colour": input.title_colour,
                "title_bg_colour": input.title_bg_colour,
                "text_colour": input.text_colour,
                "text_bg_colour": input.text_bg_colour,
                "padding_horizontal": input.padding_horizontal,
                "padding_vertical": input.padding_vertical,
            })),
        };

        match output {
            Ok(badge) => Ok(badge),
            Err(_err) => {
                error!("{}", _err);
                Err("failed to render badge".into())
            }
        }
    }
}

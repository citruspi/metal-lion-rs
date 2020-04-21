#[derive(Debug, Deserialize)]
pub struct SvgBadgeInput {
    pub title: String,
    pub text: String,
    pub font_face: Option<minutiae::FontFace>,
    pub font_size: Option<minutiae::FontSize>,
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

    pub fn render_svg(&self, input: SvgBadgeInput) -> String {
        let bbox = self.opts.render_dataset.bounding_box(
            &format!("{} - {}", input.title, input.text),
            minutiae::BoundingBoxRenderOptions {
                face: "Verdana".to_string(),
                size: "100".to_string(),
            },
        );

        match bbox {
            Some(v) => format!("{} - {} ({} x {})", input.title, input.text, v[0], v[1]).into(),
            None => "failed to render badge".into(),
        }
    }
}

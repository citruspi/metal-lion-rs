#[derive(Debug, Deserialize)]
pub struct SvgBadgeInput {
    pub title: String,
    pub text: String,
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
}

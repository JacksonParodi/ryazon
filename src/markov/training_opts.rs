use crate::{misc::constant, ryazon::RyazonArgs};
use std::path::PathBuf;

pub struct TrainingOptions {
    pub order: u8,
    pub path: PathBuf,
    pub remove_urls: bool,
    pub remove_punctuation: bool,
    pub add_punctuation: Option<String>,
}

impl From<RyazonArgs> for TrainingOptions {
    fn from(args: RyazonArgs) -> Self {
        Self {
            order: args.order.unwrap_or(constant::DEFAULT_ORDER),
            path: args.texts_path,
            remove_urls: args.remove_urls,
            remove_punctuation: args.remove_punctuation,
            add_punctuation: args.add_punctuation,
        }
    }
}

use crate::{misc::constant, ryazon::RyazonArgs};

#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub seed: Option<String>,
    pub terminator: Option<String>,
    pub max_words: usize,
    pub min_words: usize,
}

impl From<RyazonArgs> for GenerationOptions {
    fn from(args: RyazonArgs) -> Self {
        Self {
            seed: args.seed,
            terminator: args.terminator,
            max_words: args.max_words.unwrap_or(constant::DEFAULT_MAX_WORDS),
            min_words: args.min_words.unwrap_or(constant::DEFAULT_MIN_WORDS),
        }
    }
}

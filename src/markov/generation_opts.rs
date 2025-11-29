use crate::{misc::constant, ryazon::RyazonArgs};

#[derive(Debug, Clone)]
pub struct GenerationOptions {
    pub seed: Option<String>,
    pub terminator: Option<String>,
    pub max_words: usize,
    pub min_words: usize,
    pub iterations: u16,
}

impl From<RyazonArgs> for GenerationOptions {
    fn from(args: RyazonArgs) -> Self {
        Self {
            seed: args.seed,
            terminator: args.terminator,
            max_words: args.max_words.unwrap_or(constant::DEFAULT_MAX_WORDS),
            min_words: args.min_words.unwrap_or(constant::DEFAULT_MIN_WORDS),
            iterations: args.iterations.unwrap_or(constant::DEFAULT_ITERATIONS),
        }
    }
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            seed: Some("default_seed".to_string()),
            terminator: None,
            max_words: constant::DEFAULT_MAX_WORDS,
            min_words: constant::DEFAULT_MIN_WORDS,
            iterations: constant::DEFAULT_ITERATIONS,
        }
    }
}

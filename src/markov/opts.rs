use crate::misc::constant;

pub struct GenerationOptions {
    pub seed: Option<String>,
    pub terminator: Option<String>,
    pub max_words: usize,
    pub min_words: usize,
}

impl GenerationOptions {
    pub fn new(
        seed: Option<String>,
        terminator: Option<String>,
        max_words: usize,
        min_words: usize,
    ) -> Self {
        Self {
            seed,
            terminator,
            max_words,
            min_words,
        }
    }
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            seed: None,
            terminator: None,
            max_words: constant::DEFAULT_MAX_WORDS,
            min_words: constant::DEFAULT_MIN_WORDS,
        }
    }
}

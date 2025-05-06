use crate::{
    markov::{GenerationOptions, TrainingOptions},
    ryazon::RyazonError,
};
use rand::Rng;
use serde_json;
use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
};

#[derive(Debug)]
pub struct MarkovChain {
    chain: HashMap<Vec<String>, HashMap<String, usize>>,
    order: u8,
}

impl MarkovChain {
    pub fn new(opts: TrainingOptions) -> Self {
        let mut chain = MarkovChain {
            chain: HashMap::new(),
            order: opts.order,
        };

        // TODO: check what kind of JSON value the file is, convert it to Vec<String> if needed

        let file = match File::open(&opts.path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("failed to open JSON file: {:?}", opts.path);
                std::process::exit(1)
            }
        };

        let json_value: serde_json::Value = match serde_json::from_reader(file) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("failed to parse JSON file: {:?}", opts.path);
                std::process::exit(1)
            }
        };

        // TODO: more robust validation for wacky JSON data
        let texts: Vec<String> = match json_value {
            serde_json::Value::String(s) => vec![s],
            serde_json::Value::Array(arr) => arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            serde_json::Value::Object(obj) => obj
                .values()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            _ => {
                eprintln!("unsupported JSON value type: {:?}", json_value);
                std::process::exit(1)
            }
        };

        chain.train(&texts, &opts);
        chain
    }

    pub fn train(&mut self, texts: &[String], opts: &TrainingOptions) {
        let mut concatenated_text = String::new();

        for text in texts {
            if text.trim().is_empty() {
                continue;
            }

            let mut words: Vec<String> =
                text.split_whitespace().map(|s| s.to_lowercase()).collect();

            if opts.remove_urls {
                words.retain(|word| {
                    !word.starts_with("http://")
                        && !word.starts_with("https://")
                        && !word.starts_with("www.")
                });
            }

            if opts.remove_punctuation {
                for word in &mut words {
                    word.retain(|c| !c.is_ascii_punctuation());
                }
            }

            if let Some(punctuator) = &opts.add_punctuation {
                if !words
                    .last()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .is_ascii_punctuation()
                {
                    let mut new_last_word = words.last().unwrap().clone();
                    new_last_word.push_str(punctuator);
                    words.pop();
                    words.push(new_last_word);
                }
            }

            concatenated_text.push_str(&words.join(" "));
            concatenated_text.push_str(" ");
        }

        self.add_text(&concatenated_text);
    }

    pub fn add_text(&mut self, text: &str) {
        let mut words: Vec<String> = text.split_whitespace().map(|s| s.to_lowercase()).collect();

        if words.len() <= self.order.into() {
            return;
        }

        // if the last word does not end with a punctuation mark, add a period
        // possible make this a flag in the future
        if !words
            .last()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .is_ascii_punctuation()
        {
            let mut new_last_word = words.last().unwrap().clone();
            new_last_word.push_str(".");
            words.pop();
            words.push(new_last_word);
        }

        for i in 0..=words.len() - (usize::from(self.order) + 1) {
            let state: Vec<String> = words[i..i + usize::from(self.order)]
                .iter()
                .cloned()
                .collect();
            let next_word = words[i + usize::from(self.order)].clone();

            match self.chain.entry(state) {
                Entry::Occupied(mut entry) => {
                    let next_words = entry.get_mut();
                    *next_words.entry(next_word).or_insert(0) += 1;
                }
                Entry::Vacant(entry) => {
                    let mut next_words = HashMap::new();
                    next_words.insert(next_word, 1);
                    entry.insert(next_words);
                }
            }
        }
    }

    pub fn generate(&self, opts: &GenerationOptions) -> Result<String, RyazonError> {
        if self.chain.is_empty() {
            return Err(RyazonError::EmptyChain);
        }

        let mut rng = rand::rng();

        let mut current_state = match opts.seed {
            Some(ref seed) => {
                let matching_states: Vec<&Vec<String>> = self
                    .chain
                    .keys()
                    .filter(|state| {
                        !state.is_empty() && state[0].to_lowercase() == seed.to_lowercase()
                    })
                    .collect();

                if matching_states.is_empty() {
                    let all_states_with_seed: Vec<&Vec<String>> = self
                        .chain
                        .keys()
                        .filter(|state| state.contains(&seed.to_lowercase()))
                        .collect();

                    if all_states_with_seed.is_empty() {
                        eprintln!(
                            "seed word '{}' not found in chain, using random state",
                            seed
                        );
                        let keys: Vec<&Vec<String>> = self.chain.keys().collect();
                        keys[rng.random_range(0..keys.len())].clone()
                    } else {
                        all_states_with_seed[rng.random_range(0..all_states_with_seed.len())]
                            .clone()
                    }
                } else {
                    matching_states[rng.random_range(0..matching_states.len())].clone()
                }
            }
            None => {
                let keys: Vec<&Vec<String>> = self.chain.keys().collect();
                keys[rng.random_range(0..keys.len())].clone()
            }
        };

        let mut result: Vec<String> = current_state.clone();
        let mut terminator_found = false;

        for _ in 0..opts.max_words - usize::from(self.order) {
            if let Some(next_words) = self.chain.get(&current_state) {
                let total_count: usize = next_words.values().sum();
                let mut random_val = rng.random_range(0..total_count);

                let mut selected_word = String::new();
                for (word, &count) in next_words {
                    if random_val < count {
                        selected_word = word.clone();
                        break;
                    }
                    random_val -= count;
                }

                result.push(selected_word.clone());

                if opts.terminator.is_some() && result.len() >= opts.min_words {
                    if selected_word.ends_with(opts.terminator.as_ref().unwrap()) {
                        terminator_found = true;
                        break;
                    }
                }

                current_state.remove(0);
                current_state.push(selected_word);
            } else {
                break;
            }
        }

        match opts.terminator {
            Some(ref _t) => {
                if terminator_found {
                    Ok(result.join(" "))
                } else {
                    Err(RyazonError::TerminatorNotFound(format!(
                        "terminator '{}' not found in generated text",
                        opts.terminator.as_ref().unwrap()
                    )))
                }
            }
            _ => Ok(result.join(" ")),
        }
    }
}

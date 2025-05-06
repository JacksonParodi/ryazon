use crate::markov::GenerationOptions;
use rand::Rng;
use serde_json;
use std::{
    collections::{HashMap, hash_map::Entry},
    fs::File,
    path::PathBuf,
};

#[derive(Debug)]
pub struct MarkovChain {
    chain: HashMap<Vec<String>, HashMap<String, usize>>,
    order: usize,
}

impl MarkovChain {
    pub fn new(order: usize, texts_path: PathBuf) -> Self {
        let mut chain = MarkovChain {
            chain: HashMap::new(),
            order,
        };

        let texts: Vec<String> =
            serde_json::from_reader(File::open(texts_path).expect("Failed to open JSON file"))
                .expect("Failed to parse JSON");

        chain.train(&texts);
        chain
    }

    pub fn train(&mut self, texts: &[String]) {
        for text in texts {
            self.add_text(text);
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let mut words: Vec<String> = text.split_whitespace().map(|s| s.to_lowercase()).collect();

        if words.len() <= self.order {
            return;
        }

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

        for i in 0..=words.len() - (self.order + 1) {
            let state: Vec<String> = words[i..i + self.order].iter().cloned().collect();
            let next_word = words[i + self.order].clone();

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

    pub fn generate(&self, opts: GenerationOptions) -> Option<String> {
        if self.chain.is_empty() {
            return None;
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

        for _ in 0..opts.max_words - self.order {
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
                        println!("terminator found in result: {}", result.join(" "));
                        break;
                    }
                }

                current_state.remove(0);
                current_state.push(selected_word);
            } else {
                break;
            }
        }

        Some(result.join(" "))
    }
}

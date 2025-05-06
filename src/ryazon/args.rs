use crate::ryazon::RyazonError;
use clap::parser::ArgMatches;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RyazonArgs {
    pub texts_path: PathBuf,
    pub output_path: Option<PathBuf>,

    pub order: Option<u8>,
    pub seed: Option<String>,
    pub max_words: Option<usize>,
    pub min_words: Option<usize>,
    pub terminator: Option<String>,

    pub remove_urls: bool,
    pub remove_punctuation: bool,

    pub add_punctuation: Option<String>,
}

impl From<ArgMatches> for RyazonArgs {
    fn from(matches: ArgMatches) -> Self {
        let texts_path_raw = match matches.get_one::<String>("training_texts") {
            Some(texts_path_str) => texts_path_str,
            None => {
                eprintln!("no path provided: {:?}", RyazonError::NoPath);
                std::process::exit(1);
            }
        };
        let texts_path = PathBuf::from(texts_path_raw);

        let output_path = match matches.get_one::<String>("output_json") {
            Some(output_path_str) => Some(PathBuf::from(output_path_str)),
            None => None,
        };

        let order = match matches.get_one::<String>("order") {
            Some(o) => Some(o.parse::<u8>().unwrap()),
            None => None,
        };
        let seed = match matches.get_one::<String>("seed_word") {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        let max_words = match matches.get_one::<String>("max_words") {
            Some(m) => Some(m.parse::<usize>().unwrap()),
            None => None,
        };
        let min_words = match matches.get_one::<String>("min_words") {
            Some(m) => Some(m.parse::<usize>().unwrap()),
            None => None,
        };
        let terminator = match matches.get_one::<String>("terminator") {
            Some(t) => Some(t.to_string()),
            None => None,
        };

        let remove_urls = matches.get_flag("remove_urls");
        let remove_punctuation = matches.get_flag("remove_punctuation");

        let add_punctuation = match matches.get_one::<String>("add_punctuation") {
            Some(a) => Some(a.to_string()),
            None => None,
        };

        RyazonArgs {
            output_path,
            texts_path,
            order,
            seed,
            max_words,
            min_words,
            terminator,
            remove_urls,
            remove_punctuation,
            add_punctuation,
        }
    }
}

mod markov;
mod misc;

use clap::{Command, arg};

fn main() {
    let matches = Command::new("ryazon")
        .version("0.0")
        .author("jackson parodi")
        .about("a markov chain text generator")
        .arg(arg!(training_texts: -t --texts <FILE> "path to the json file of texts"))
        .arg(arg!(order: -o --order [NUMBER] "order of the markov chain"))
        .arg(arg!(seed_word: -s --seed [SEED] "seed words to start the generation"))
        .arg(arg!(max_words: -x --max_words [NUMBER] "maximum words to generate"))
        .arg(arg!(min_words: -n --min_words [NUMBER] "minimum words to generate"))
        .arg(arg!(terminator: -m --terminator [CHAR] "terminator character"))
        .get_matches();

    let texts_path_raw = match matches.get_one::<String>("training_texts") {
        Some(texts_path_str) => texts_path_str,
        None => {
            eprintln!("Error: No training texts provided.");
            std::process::exit(1);
        }
    };

    let texts_path = std::path::PathBuf::from(texts_path_raw);

    let order = match matches.get_one::<String>("order") {
        Some(order_str) => order_str.parse::<usize>().unwrap(),
        None => misc::constant::DEFAULT_ORDER,
    };

    let seed = match matches.get_one::<String>("seed_word") {
        Some(seed_str) => Some(seed_str.to_string()),
        None => None,
    };

    let max_words = match matches.get_one::<String>("max_words") {
        Some(max_words_str) => max_words_str.parse::<usize>().unwrap(),
        None => misc::constant::DEFAULT_MAX_WORDS,
    };

    let min_words = match matches.get_one::<String>("min_words") {
        Some(min_words_str) => min_words_str.parse::<usize>().unwrap(),
        None => misc::constant::DEFAULT_MIN_WORDS,
    };

    let terminator = match matches.get_one::<String>("terminator") {
        Some(terminator_str) => Some(terminator_str.to_string()),
        None => None,
    };

    let chain = markov::MarkovChain::new(order, texts_path);

    let options = markov::GenerationOptions::new(seed, terminator, max_words, min_words);

    match chain.generate(options) {
        Some(generated_text) => {
            println!("Generated text: {}", generated_text);
        }
        None => {
            println!("Failed to generate text.");
        }
    }
}

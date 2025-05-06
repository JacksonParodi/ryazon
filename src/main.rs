use clap::{Command, arg};

fn main() {
    let matches = Command::new("ryazon")
        .version("0.0")
        .author("jackson parodi")
        .about("a markov chain text generator")
        .arg(arg!(training_texts: -t --texts <FILE> "path to the json file of texts"))
        .arg(arg!(seed_word: -s --seed [SEED] "seed words to start the generation"))
        .arg(arg!(max_words: -x --max_words [NUMBER] "maximum words to generate"))
        .arg(arg!(min_words: -n --min_words [NUMBER] "minimum words to generate"))
        .arg(arg!(terminator: -m --terminator [CHAR] "terminator character"))
        .get_matches();

    println!(
        "seed: {:?}",
        matches.get_one::<String>("seed_word").expect("seed")
    );
}

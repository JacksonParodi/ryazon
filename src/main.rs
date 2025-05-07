mod markov;
mod misc;
mod ryazon;

use markov::{GenerationOptions, MarkovChain, TrainingOptions};
use ryazon::{RyazonArgs, RyazonError, RyazonOutput};

use clap::{Command, arg};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ryazon")
        .version("0.1.4")
        .author("jackson parodi")
        .about("a markov chain text generator")
        .arg(arg!(training_texts: <FILE> "path to the json file of texts"))
        .arg(arg!(output_json: -o --output [FILE] "path to the output JSON file"))
        .arg(arg!(order: -r --order [NUMBER] "order of the markov chain"))
        .arg(arg!(seed_word: -s --seed [SEED] "seed words to start the generation"))
        .arg(arg!(max_words: -x --max_words [NUMBER] "maximum words to generate"))
        .arg(arg!(min_words: -n --min_words [NUMBER] "minimum words to generate, used with terminator"))
        .arg(arg!(terminator: -m --terminator [CHAR] "terminator character"))
        .arg(arg!(remove_urls: -u --remove_urls "remove URL links from the training texts"))
        .arg(arg!(remove_punctuation: -p --remove_punctuation "remove all punctuation marks from the training texts"))
        .arg(arg!(add_punctuation: -a --add_punctuation [CHAR] "add punctuation to the end of each training text"))
        .get_matches();

    let args = RyazonArgs::from(matches);
    let train_opts = TrainingOptions::from(args.clone());
    let gen_opts = GenerationOptions::from(args.clone());

    if gen_opts.max_words < gen_opts.min_words {
        return Err(Box::new(RyazonError::MaxMinWords));
    }

    let chain = MarkovChain::new(train_opts);

    let output = match chain.generate(&gen_opts) {
        Ok(generated_text) => RyazonOutput::Success(generated_text),
        Err(e) => match e {
            RyazonError::TerminatorNotFound(e) => {
                let mut result = RyazonOutput::Error(RyazonError::TerminatorNotFound(e.clone()));

                for i in 0..misc::constant::TERMINATOR_RETRY_LIMIT {
                    if i >= misc::constant::TERMINATOR_RETRY_LIMIT - 1 {
                        return Err(Box::new(RyazonError::TerminatorNotFound(e)));
                    }

                    match chain.generate(&gen_opts) {
                        Ok(generated_text) => {
                            result = RyazonOutput::Success(generated_text);
                            break;
                        }
                        Err(e) => match e {
                            RyazonError::TerminatorNotFound(_msg) => {
                                continue;
                            }
                            _ => {
                                return Err(Box::new(RyazonError::IoError(e.to_string())));
                            }
                        },
                    }
                }

                result
            }
            RyazonError::IoError(err) => RyazonOutput::Error(RyazonError::IoError(err)),
            _ => RyazonOutput::Error(RyazonError::IoError(
                std::io::Error::new(std::io::ErrorKind::Other, "unexpected error").to_string(),
            )),
        },
    };

    let output_json = match serde_json::to_string_pretty(&output) {
        Ok(json) => json,
        Err(e) => {
            return Err(Box::new(RyazonError::IoError(e.to_string())));
        }
    };

    match &args.output_path {
        Some(path) => match std::fs::write(path, output_json) {
            Ok(_) => {
                return Ok(println!("successfully wrote to: {:?}", path));
            }
            Err(e) => {
                eprintln!("Error writing to file: {}", e);
                std::process::exit(1);
            }
        },
        None => match output {
            RyazonOutput::Success(generated_text) => {
                return Ok(println!("{}", generated_text));
            }
            RyazonOutput::Error(err) => {
                return Err(Box::new(err));
            }
        },
    }
}

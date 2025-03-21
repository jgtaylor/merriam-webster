use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use merriam_webster_http::{DefinitionResponse, MerriamWebsterClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let api_key = env::var("MERRIAM_WEBSTER_API_KEY")?;
    let client = MerriamWebsterClient::new(api_key.into());

    let mut rng = thread_rng();
    // let top_words = client.top_words().await?.data.words;
    // let word = top_words.choose(&mut rng).unwrap();
    let words = read_lines("/etc/dictionaries-common/words").unwrap();
    let word = words.choose(&mut rng).unwrap();
    println!("{word}");
    // let word = "censure".to_string();
    let defs = client.collegiate_definition(word.to_string()).await?;
    match defs {
        DefinitionResponse::Entries(entries) => {
            let def = entries
                .first()
                .unwrap_or_else(|| panic!("No definition found for {}", word));
            let shortdefs = def.shortdef.as_ref().unwrap();

            println!(
                "The short definitions of \x1b[33;1m{word}\x1b[0m are:\n\n{}",
                shortdefs
                    .iter()
                    .map(|d| format!("\x1b[0;34m-\x1b[0;0m {d}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
        DefinitionResponse::PotentialWords(list) => {
            println!(
                "Perhaps you meant one of the following words?:\n{}",
                list.join("\n")
            );
        }
    }

    Ok(())
}

pub fn read_lines(filename: &str) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    Ok(reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
}

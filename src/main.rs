use clap::Parser;
use rand::{seq::SliceRandom, thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::io::{self, Write};

#[derive(Parser)]
struct Args {
    characters: String,
    count: usize,
    seed: Option<u64>,
}

fn main() {
    let mut prompts_completed: u32 = 0;
    let mut mistakes: u32 = 0;

    let args = Args::parse();

    let mut rng: Box<dyn RngCore> = args
        .seed
        .map_or(Box::new(thread_rng()) as Box<dyn RngCore>, |seed| {
            Box::new(ChaChaRng::seed_from_u64(seed))
        });

    let mut chars: Vec<char> = args.characters.chars().collect();
    loop {
        let mut word = String::new();
        while word.len() < args.count {
            chars.shuffle(&mut rng);
            word += &chars.get(0).unwrap().to_string();
            if word.len() == 1 {
                word = word.trim_start().into();
            } else if word.len() == args.count {
                word = word.trim_end().into();
            }
        }

        loop {
            println!(
                "completed: {}, mistakes were made: {}",
                prompts_completed, mistakes
            );
            println!("{:-<1$}", "", args.count as usize);
            println!("{}", word);
            println!("");
            println!("{:-<1$}", "", args.count as usize);
            println!("{esc}[3F", esc = 27 as char);

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if word == input {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                prompts_completed += 1;
                break;
            }
            println!("wrong!");
            mistakes += 1;
        }
    }
}

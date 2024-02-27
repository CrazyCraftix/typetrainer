use clap::Parser;
use rand::{seq::SliceRandom, thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::io::{self, Write};

#[derive(Parser)]
struct Args {
    characters: String,
    count: u32,
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let mut rng: Box<dyn RngCore> = args.seed.map_or(Box::new(thread_rng()) as Box<dyn RngCore>, |seed| Box::new(ChaChaRng::seed_from_u64(seed)));

    let mut chars: Vec<char> = args.characters.chars().collect();
    loop {
        let mut word = String::new();
        for _ in 0..args.count {
            chars.shuffle(&mut rng);
            word += &chars.get(0).unwrap().to_string();
        }

        loop {
            println!("'{}'", word);
            print!(" ");

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if word == input {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                break;
            }
            println!("wrong!");
        }
    }
}

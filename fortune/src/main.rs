use clap::Parser;
use rand::random_range;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DEFAULT_FORTUNE: &str = "./fortunes";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'm')]
    m_reg: Option<Regex>,

    paths: Vec<String>,
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut filelines = Vec::new();

    if args.paths.is_empty() {
        // DEFAULT_FORTUNE を読む
        for result in BufReader::new(File::open(DEFAULT_FORTUNE)?).lines() {
            filelines.push(result?);
        }
    } else {
        // args.paths を読む
        for path in &args.paths {
            for result in BufReader::new(File::open(&path)?).lines() {
                filelines.push(result?);
            }
        }
    } //if args.paths.is_empty()

    let fortune_line: Vec<&[String]> = filelines
        .split(|x| x == "%")
        .filter(|x| !x.is_empty())
        .collect();

    if let Some(m_reg) = &args.m_reg {
        for lines in fortune_line {
            if lines.iter().any(|line| m_reg.is_match(line)) {
                for line in lines {
                    println!("{}", line);
                }
                println!("%");
            }
        }
    } else {
        let n: u32 = random_range(0..fortune_line.len() as u32);
        for line in fortune_line[n as usize] {
            println!("{}", line);
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("{}", e);
    }
}

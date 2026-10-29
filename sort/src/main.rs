use clap::Parser;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
enum SortOption {
    Reverse,
    Numeric,
    Unique,
    Key(usize),
}
 */

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r', long = "reverse")]
    reverse: bool,

    #[arg(short = 'n', long = "numeric-sort")]
    numeric: bool,

    #[arg(short = 'u', long = "unique")]
    unique: bool,

    #[arg(short = 'k', long = "key")]
    key: Option<usize>,

    path: String,
}

fn is_numeric(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

fn run(args: &Args) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut filelines = Vec::new();
    for result in BufReader::new(File::open(&args.path)?).lines() {
        filelines.push(result?);
    }

    if args.numeric {
        filelines.sort_by_key(|s| {
            if !is_numeric(s) {
                0
            } else {
                s.parse::<u32>().unwrap()
            }
        });
    } else if args.reverse {
        //文字列を降順にソート
        filelines.sort_by_key(|s| Reverse(s.clone()));
    } else if args.unique {
        //ダブり行を削除
        filelines.sort();
        filelines.dedup();
    } else if args.key.is_some() {
        let keyval = args.key.unwrap();

        if keyval == 0 {
            //panic!("field param error..");
            return Err(std::io::Error::other(" field param error.. ").into());
        }

        filelines.sort_by_key(|s| {
            let split_line: Vec<&str> = s.split_whitespace().collect();
            //split_line[keyval - 1].to_string()
            split_line.get(keyval - 1).unwrap_or(&"").to_string()
        });
    } else {
        //文字列を昇順にソート
        filelines.sort();
    }

    Ok(filelines)
}

fn printlines(v: &[String]) {
    for line in v {
        println!("{}", line);
    }
}

fn main() {
    let args = Args::parse();

    match run(&args) {
        Ok(v) => {
            //暗黙の型変換：&Vec[T]->&[T]
            printlines(&v);
        }
        Err(e) => {
            eprintln!("file handling error ={}", e);
        }
    }
}

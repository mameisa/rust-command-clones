use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Parser, Debug)]
struct Args {
    path: String,
}

fn run(args: &Args) -> Result<Vec<String>, Box<dyn std::error::Error>>{

    let mut filelines = Vec::new();
    for result in BufReader::new(File::open(&args.path)?).lines() {
        filelines.push(result?);
    }

    //文字列を昇順にソート
    filelines.sort();


    Ok(filelines)
}

fn printlines(v:&[String]){
    for line in v{
        println!("{}",line);
    }
}

fn main() {
   let args = Args::parse();
   match run(&args){
    Ok(v)=>{
        //暗黙の型変換：&Vec[T]->&[T]
        printlines(&v);
    },
    Err(e)=>{
        eprintln!("file handling error ={}",e);
    }
   }
}

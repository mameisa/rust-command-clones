use clap::Parser;
use std::io::{self, Read};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'n', long = "max-args")]
    max_args: Option<usize>,

    command: String,

    fixarg: Vec<String>,
}

fn remove_empty_input(v: Vec<String>) -> Vec<String> {
    v.into_iter().filter(|s| !s.trim().is_empty()).collect()
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    //ここに標準入力を受け取って、解析する処理を書く。
    if let Some(max_size) = args.max_args {
        if max_size == 0 {
            return Err("-n must be greater than 0".into());
        }

        let v = read_buffer_by_max(max_size);

        for elm in v {
            let v = remove_empty_input(elm);
            if !v.is_empty() {
                process_cmd(args, &v)?;
            }
        }
    } else {
        //let input = read_buffer();
        let input = remove_empty_input(read_buffer());
        if !input.is_empty() {
            process_cmd(args, &input)?;
        }
    }

    Ok(())
}

fn read_buffer() -> Vec<String> {
    let mut input = String::new();
    // 標準入力からすべてをStringに読み込む
    io::stdin().read_to_string(&mut input).unwrap();

    /*
        let v = input.split("\n").collect::<Vec<&str>>();
    */
    input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn read_buffer_by_max(max_size: usize) -> Vec<Vec<String>> {
    let mut input = String::new();
    // 標準入力からすべてをStringに読み込む
    io::stdin().read_to_string(&mut input).unwrap();
    let mut ret: Vec<Vec<String>> = Vec::new();
    let v = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    //
    for elm in v.chunks(max_size) {
        ret.push(elm.to_vec());
    }
    return ret;
}

fn process_cmd(args: &Args, pipe_s: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new(&args.command)
        .args(&args.fixarg)
        .args(pipe_s)
        .output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!("command failed: {:?}", output.status.code())).into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(&args)
}

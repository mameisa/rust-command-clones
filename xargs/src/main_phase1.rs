
use clap::Parser;
use std::io::{self, Read};
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    command: String,
    fixarg: Vec<String>,
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {

    //ここに標準入力を受け取って、解析する処理を書く。
    let input = read_buffer();

    process_cmd(&args, &input)?;

    Ok(())
}

fn read_buffer() -> Vec<String> {

    let mut input = String::new();
    // 標準入力からすべてをStringに読み込む
    io::stdin().read_to_string(&mut input).unwrap();

    /* 
        let v = input.split("\n").collect::<Vec<&str>>();
    */

    input.lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()

}

fn process_cmd(
    args: &Args,
    pipe_s: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    let output = Command::new(&args.command)
        .args(&args.fixarg)
        .args(pipe_s)
        .output()?;

    // ステータスコードの取得 (Option<i32>)
    /*
    if let Some(code) = output.status.code() {
        println!("exit code: {}", code);
    }
    */

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(
            format!("command failed: {:?}", output.status.code())
            ).into()
        )
    }
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(&args)
}

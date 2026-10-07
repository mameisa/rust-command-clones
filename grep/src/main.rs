
use std::fs::File;
use std::io::{self, BufRead, BufReader};
struct Config {
    pattern: String,
    input: Option<String>,
}

fn parse_args() -> Result<Config, String> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Usage: grep <pattern> [input_file]".to_string());
    }

    match args.len() {
        2 => {
            Ok(
                Config {
                    pattern: args[1].clone(),
                    input: None,
                }
            )
        }
        3 => {
            Ok(
                Config {
                    pattern: args[1].clone(),
                    input: Some(args[2].clone()),
                }
            )
        }
        _ =>{Err("Usage: grep <pattern> [input_file]".to_string())}

    }//match args.len() 

    //Ok(config)

}



fn open_input(
    path: &Option<String>,
) -> Result<Box<dyn BufRead>, String> {
    match path {
        Some(p) => {
            let file = File::open(p)
                .map_err(|e| format!("Failed to open {}: {}", p, e))?;

            Ok(Box::new(BufReader::new(file)))
        }
        None => {
            Ok(Box::new(BufReader::new(io::stdin())))
        }
    }
}


fn grep<R: BufRead>(reader: R,pattern: &str) 
-> Result<(), String> {
    
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}


/*
fn grep<R: std::io::Read>(reader: R, pattern: &str) -> Result<(), String> {
    Ok(())
}
*/

fn run(config: &Config) -> Result<(), String> {

    let reader = open_input(&config.input)?;

    grep(reader, &config.pattern)?;

    Ok(())
}

fn main() {
   let config = parse_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    run(&config).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}

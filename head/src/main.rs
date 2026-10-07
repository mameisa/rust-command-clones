use std::env;
use std::io;
use std::io::{BufRead,Read,Write};



fn main() -> io::Result<()> {

   
    let args: Vec<String> = env::args().collect();

    /*
    for arg in &args {
        println!("{}", arg);
    }
    */

    //オプションチェック
    if args.len() < 4 {
        println!("usage: head [-n|-c] num file");
        std::process::exit(1);
    }

    enum ReadMode {
        Line,
        Byte,
    }

    let mode = match args[1].as_str() {
        "-n" => ReadMode::Line,
        "-c" => ReadMode::Byte,
        _ => {
            eprintln!("invalid option");
            std::process::exit(1);
        }
    };

    let read_num: isize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("invalid number");
            std::process::exit(1);
        }
    };

    if read_num < 0 {
        eprintln!("negative value is not allowed");
        std::process::exit(1);
    }

    //対象ファイル読み込み
    for file_name in &args[3..] {
        let file = std::fs::File::open(file_name);
        let reader = match file {
            Ok(f) => std::io::BufReader::new(f),
            Err(e) => {
                eprintln!("failed to open file {}: {}", file_name, e);
                continue;
            }
        };

        match mode {
            ReadMode::Line => {
                for (i, line) in reader.lines().enumerate() {
                    if i as isize >= read_num {
                        break;
                    }
                    match line {
                        Ok(l) => println!("{}", l),
                        Err(e) => eprintln!("failed to read line: {}", e),
                    }
                }
            }
            ReadMode::Byte => {
                let mut buffer: Vec<u8> =Vec::new();
                match reader.take(read_num as u64).read_to_end(&mut buffer) {
                    Ok(_) => std::io::stdout().write_all(&buffer)?,
                    Err(e) => eprintln!("failed to read bytes: {}", e),
                }
            }
        }
    }

    Ok(())

}

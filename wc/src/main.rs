
use std::env;
use std::io::BufRead;

fn main() {

    let args: Vec<String> = env::args().collect();

    //オプションチェック
    if args.len() > 3 || args.len() < 2 {
        println!("usage: wc [-l|-w|-c] file_name");
        std::process::exit(1);
    }

    enum ReadMode {
        Line,
        Word,
        Byte,
        ALL,
    }

    
    let mut mode = ReadMode::ALL;
    if args.len() > 2 {  
        mode = match args[1].as_str() {
            "-l" => ReadMode::Line,
            "-w" => ReadMode::Word,
            "-c" => ReadMode::Byte,
            _ => {
                eprintln!("invalid option");
                std::process::exit(1);
            }
        };
    }//if args.len() > 2

    let file_path = if args.len() > 2 { &args[2] } else { &args[1] };

    let file = std::fs::File::open(file_path.as_str());
    let reader = match file {
            Ok(f) => std::io::BufReader::new(f),
            Err(e) => {
                eprintln!("failed to open file {}: {}", file_path.as_str(), e);
                std::process::exit(1);
        }
    };

    match mode {
        ReadMode::Line => {
            let mut line_count = 0;
            for line in reader.lines() {
                match line {
                    Ok(_) => line_count += 1,
                    Err(e) => eprintln!("failed to read line: {}", e),
                }
            }
            println!("{}", line_count);
        }
        ReadMode::Word => {
            let mut word_count = 0;
            for line in reader.lines() {
                match line {
                    Ok(l) => word_count += l.split_whitespace().count(),
                    Err(e) => eprintln!("failed to read line: {}", e),
                }
            }
            println!("{}", word_count);
        }
        ReadMode::Byte => {
            println!("{}", std::fs::metadata(file_path.as_str()).unwrap().len()); 
        }
        ReadMode::ALL => {
            let mut line_count = 0;
            let mut word_count = 0;
            let byte_count = std::fs::metadata(file_path).unwrap().len();
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        line_count += 1;
                        word_count += l.split_whitespace().count();
                    }
                    Err(e) => eprintln!("failed to read line: {}", e),
                }
            }
            println!("{} {} {}", line_count, word_count, byte_count);
        }
    }
}

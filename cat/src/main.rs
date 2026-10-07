use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn main() -> io::Result<()> {
    
    let mut count_empty_line = false;
    let args: Vec<String> = env::args().collect();

    //対象ファイル名取得
    let file_name = &args[1];

    //オプションの取得
    for arg in &args[2..] {
        if *arg == "-n" {
            count_empty_line = true;
        }
    }

    //ファイルハンドルの取得
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    //行番号カウンター
    let mut counter_lines = 1;
    
    for line in reader.lines() {
        let line_strings = line?;
        if line_strings.trim().is_empty() && !count_empty_line {
            println!("{}", line_strings);
        }else{
            println!("{} {}", counter_lines,line_strings);
            counter_lines += 1;
        }
    }
    Ok(())
}


use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// 指定されたインデックスが、ユーザーの指定した範囲に含まれるか判定
fn is_selected(idx: usize, targets: &[usize], ranges: &[(usize, usize)]) -> bool {
    if targets.contains(&idx) { return true; }
    for &(start, end) in ranges {
        if idx >= start && idx <= end { return true; }
    }
    false
}

// 引数の文字列（"1-5", "3-", "2" など）を解析する関数
fn parse_spec(spec: &str, singles: &mut Vec<usize>, ranges: &mut Vec<(usize, usize)>) {
    for part in spec.split(',') {
        if part.is_empty() { continue; }
        if part.contains('-') {
            let s: Vec<&str> = part.split('-').collect();
            if s.len() == 2 {
                let start = if s[0].is_empty() { 1 } else {                   
                    match s[0].parse::<usize>(){
                       Ok(pos) =>{pos},
                       Err(_er)=>{eprintln!("start position failed!,so will be startposition = 1 ");1},
                    }
                };
                let end = if s[1].is_empty() { usize::MAX } else { 
                    match s[1].parse::<usize>(){
                       Ok(pos) =>{pos},
                       Err(_er)=>{eprintln!("end position failed!,so will be endposition = usize::MAX ");usize::MAX},
                    }
                };
                ranges.push((start, end));
            }
        } else {     
            match part.parse::<usize>()
            {
                Ok(n) =>{singles.push(n);},
                Err(_er)=>{eprintln!("single position failed!, so this value will be ignored");},     
            };
        }//else
    }
}

// 読み込んだ行を処理して出力する共通ロジック
fn process_line(line: &str, mode: &str, delimiter: &str, target_fields: &[usize], field_ranges: &[(usize, usize)], target_bytes: &[usize], byte_ranges: &[(usize, usize)]) {
    if mode == "fields" {
        let mut output_parts = Vec::new();
        if !line.contains(delimiter){
            
            println!("{}", line);
            return;
        }else{
            let fields: Vec<&str> = line.split(delimiter).collect();
            
            for idx in 1..=fields.len() {
                if is_selected(idx, target_fields, field_ranges) {
                    output_parts.push(fields[idx - 1]);
                }
            }
            println!("{}", output_parts.join(delimiter));
        }

    } else if mode == "bytes" ||  mode == "chars" {
        let bytes = line.as_bytes();
        let mut output_bytes = Vec::new();
        for idx in 1..=bytes.len() {
            if is_selected(idx, target_bytes, byte_ranges) {
                output_bytes.push(bytes[idx - 1]);
            }
        }
        println!("{}", String::from_utf8_lossy(&output_bytes));
    }
}

fn is_overoption(args: &[String]) -> bool {
    let mut count = 0;

    for arg in args {
        if arg == "-b" || arg == "-c" || arg == "-f" {
            count += 1;
        }

        if count >= 2 {
            return true;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut mode = "";
    let mut delimiter = "\t".to_string();
    let mut target_fields = Vec::new();
    let mut target_bytes = Vec::new();
    let mut field_ranges = Vec::new();
    let mut byte_ranges = Vec::new();
    let mut file_paths = Vec::new();

    // --- 引数の解析 ---

    if is_overoption(&args) {
        eprintln!("error: -b, -c, and -f cannot be specified more than once");
        std::process::exit(1);
    }

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                if i + 1 < args.len() { delimiter = args[i + 1].clone(); i += 2; }
                else { eprintln!("エラー: -d の後に値を指定してください"); std::process::exit(1); }
            }
            "-f" => {
                if i + 1 < args.len() {
                    mode = "fields";
                    parse_spec(&args[i + 1], &mut target_fields, &mut field_ranges);
                    i += 2;
                } else { eprintln!("エラー: -f の後に値を指定してください"); std::process::exit(1); }
            }
            "-b" => {
                if i + 1 < args.len() {
                    mode = "bytes";
                    parse_spec(&args[i + 1], &mut target_bytes, &mut byte_ranges);
                    i += 2;
                } else { eprintln!("エラー: -b の後に値を指定してください"); std::process::exit(1); }
            }
            "-c" => {
                if i + 1 < args.len() {
                    mode = "chars";
                    parse_spec(&args[i + 1], &mut target_bytes, &mut byte_ranges);
                    i += 2;
                } else { eprintln!("エラー: -c の後に値を指定してください"); std::process::exit(1); }
            }
            _ => {
                // オプション以外の引数はすべてファイルパスとして扱う
                file_paths.push(args[i].clone());
                i += 1;
            }
        }
    }

    if mode.is_empty() {
        eprintln!("エラー: -f または -b オプションが必要です");
        std::process::exit(1);
    }

    // --- データの読み込みと処理 ---
    if file_paths.is_empty() || (file_paths.len() == 1 && file_paths[0] == "-") {
        // ファイル指定がない、または "-" の場合は標準入力から読み込む
        let stdin = io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            process_line(&line, mode, &delimiter, &target_fields, &field_ranges, &target_bytes, &byte_ranges);
        }
    } else {
        // 指定されたファイルを順番に処理する
        for path in file_paths {
            match File::open(&path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    for line in reader.lines().map_while(Result::ok) {
                        process_line(&line, mode, &delimiter, &target_fields, &field_ranges, &target_bytes, &byte_ranges);
                    }
                }
                Err(err) => {
                    eprintln!("cut: {}: {}", path, err);
                }
            }
        }
    }
}
 
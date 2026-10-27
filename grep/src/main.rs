
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use regex::Regex;

struct Config {
    pattern: String,
    input: Option<String>,
    line_number: bool,
    b_ignore_case: bool,
    b_invert_match:bool,
    b_word:bool,
    b_count:bool,
}

fn is_regex_match(re:&Regex,line: &str) -> bool{
    //let re = Regex::new(pattern).unwrap();
    re.is_match(line)
}

fn is_word(c:char)->bool{
    c.is_alphanumeric() || c == '_'
}

fn is_grep_word(line:&String,config:&Config,pos:usize)->bool{


    if (pos == 0 || (!is_word(line.chars().nth(pos-1).unwrap())))//前は無か非単語
            &&  (pos + config.pattern.len() == line.len() || !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()))//後ろは無か非単語
    {
        return true;
    }else{
        return false;
    }
/*
            if ( pos == 0 && pos + config.pattern.len() == line.len() )//無&無
               || (pos ==0 && !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()))//無&非単語
               || (!is_word(line.chars().nth(pos-1).unwrap()) &&  pos + config.pattern.len() == line.len())//非単語&無
               || (!is_word(line.chars().nth(pos-1).unwrap()) && !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()))//非単語&非単語
               {
                    return true;
               }else{
                    return false;
               }
*/

/*
              if pos == 0 && pos + config.pattern.len() == line.len(){
                //無&無
                return true;
              }else if pos ==0 && !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //無し&非単語
                return true;
              }else if pos ==0 && is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //無し&単語
                return false;
              }else if !is_word(line.chars().nth(pos-1).unwrap()) &&  pos + config.pattern.len() == line.len(){
                //非単語&無
                return true;
              }else if !is_word(line.chars().nth(pos-1).unwrap()) && !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //非単語&非単語
                return true;
              }else if !is_word(line.chars().nth(pos-1).unwrap()) && is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //非単語&単語
                return false;
              }else if is_word(line.chars().nth(pos-1).unwrap()) && pos + config.pattern.len() == line.len(){
                //単語&無
                return false;
              }else if is_word(line.chars().nth(pos-1).unwrap()) && !is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //単語&非単語
                return false;
              }else if is_word(line.chars().nth(pos-1).unwrap()) && is_word(line.chars().nth(pos+config.pattern.len()).unwrap()){
                //単語&単語
                return false
              }else{
                return false;
              }
*/

}

fn contains_whole_word(config:&Config,line: &String, pattern: &str) -> bool
{
    let mut start_idx=0;
    while let Some(pos) = line[start_idx..].find(pattern) {
        let now_pos = start_idx + pos;

        if is_grep_word(line,config,now_pos) {
            return true;
        }

        start_idx = now_pos + pattern.len();
    }
      return false;
}   

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => Ok(Config {
            pattern: args[1].clone(),
            input: None,
            line_number: false,//行番号表示
            b_ignore_case: false,//大文字・小文字を区別しないで抽出
            b_invert_match:false,//一致しない行の表示
            b_word:false,//grepで言う単語の検出
            b_count:false,//指定grep単語を含む行数カウント値
        }),

        3 => {
            if args[1] == "-n" {
                Ok(Config {
                    pattern: args[2].clone(),
                    input: None,
                    line_number: true,
                    b_ignore_case: false,
                    b_invert_match:false,
                    b_word:false,
                    b_count:false
                })
            } else {
                Ok(Config {
                    pattern: args[1].clone(),
                    input: Some(args[2].clone()),
                    line_number: false,
                    b_ignore_case: false,
                    b_invert_match:false,
                    b_word:false,//grepで言う単語の検出
                    b_count:false,
                })
            }
        }

        4 => {
            if args[1] == "-n" {

                Ok(Config {
                    pattern: args[2].clone(),
                    input: Some(args[3].clone()),
                    line_number: true,
                    b_ignore_case: false,
                    b_invert_match:false,
                    b_word:false,//grepで言う単語の検出
                    b_count:false,
                })
            }else if args[1] == "-i" {
                Ok(Config {
                    pattern: args[2].clone(),
                    input: Some(args[3].clone()),
                    line_number: false,
                    b_ignore_case: true,
                    b_invert_match:false,
                    b_word:false,//grepで言う単語の検出
                    b_count:false,
                })
            }else if args[1] == "-v"{
                Ok(Config {
                    pattern: args[2].clone(),
                    input: Some(args[3].clone()),
                    line_number: false,
                    b_ignore_case: false,
                    b_invert_match:true,
                    b_word:false,//grepで言う単語の検出
                    b_count:false,
                })
            }else if args[1] == "-w"{
                Ok(Config {
                    pattern: args[2].clone(),
                    input: Some(args[3].clone()),
                    line_number: false,
                    b_ignore_case: false,
                    b_invert_match:false,
                    b_word:true,//grepで言う単語の検出
                    b_count:false,
                })
            }else if args[1] == "-c"{
                Ok(Config {
                    pattern: args[2].clone(),
                    input: Some(args[3].clone()),
                    line_number: false,
                    b_ignore_case: false,
                    b_invert_match:false,
                    b_word:false,//grepで言う単語の検出
                    b_count:true,
                })                                         
            } else {
                Err("Usage: grep [-n] <pattern> [input_file]".to_string())
            }
        }
        _ => Err("Usage: grep [-n] <pattern> [input_file]".to_string()),
    }
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

fn grep<R: BufRead>(
    reader: R,
    config:&Config,
) -> Result<(), String> {

    let mut count = 0;

    let mut pattern_target = config.pattern.clone();

    if config.b_ignore_case == true {pattern_target = pattern_target.to_lowercase();}

    let re = Regex::new(&pattern_target).map_err(|e| format!("{}", e))?;
    
    for (idx, line) in reader.lines().enumerate() {

        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;

        let mut line_target = line.clone();

        if config.b_ignore_case {
            line_target = line_target.to_lowercase();
        }

        let is_match:bool =

            if config.b_word {
                contains_whole_word(config,&line_target,&pattern_target)
            } else {
                //正規表現で確認方法に変更
                //line_target.contains(&pattern_target)
                is_regex_match(&re, &line_target)
            };

        if is_match != config.b_invert_match {
            if config.b_count {
                count += 1;
            }else{
                if config.line_number{
                    println!("{}:{}", idx + 1, line);
                } else {
                    println!("{}", line);
                }
            }
        }//if is_match != config.b_invert_match        
    }

    if config.b_count {
        println!("{}", count);
    }

    Ok(())
    
}

fn run(config: &Config) -> Result<(), String> {

    let reader = open_input(&config.input)?;
    
    grep(reader,config)?;

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

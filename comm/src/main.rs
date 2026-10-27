use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};


/*
なし:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: false, suppress_file2: false, suppress_common: false }
-1:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: true, suppress_file2: false, suppress_common: false }
-2:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: false, suppress_file2: true, suppress_common: false }
-3:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: false, suppress_file2: false, suppress_common: true }
-12:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: true, suppress_file2: true, suppress_common: false }
-13:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: true, suppress_file2: false, suppress_common: true }
-23:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: false, suppress_file2: true, suppress_common: true }
-123:Args { file1: "abc.txt", file2: "def.txt", suppress_file1: true, suppress_file2: true, suppress_common: true }
*/

#[derive(Parser,Debug)]
struct Args {
    file1: String,
    file2: String,

    #[arg(short = '1')]
    suppress_file1: bool,

    #[arg(short = '2')]
    suppress_file2: bool,

    #[arg(short = '3')]
    suppress_common: bool,
}



//ファイルOpen処理
fn file_open(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut v = Vec::new();

    for line in reader.lines() {
        v.push(line?);
    }
    Ok(v)
}

#[derive(PartialEq)]
enum Detection{
    Equale,
    Syounari,
    Dainari,
}

fn detect_s(s1:&String,s2:&String)->Detection{
    if s1 == s2{
        Detection::Equale
    }else if s1<s2{
        Detection::Syounari
    }else{
        Detection::Dainari
    }
}
//
fn make_lines(
    v1: &Vec<String>,
    v2: &Vec<String>,
) -> (Vec<String>, Vec<String>, Vec<String>) {

    let mut i = 0;
    let mut j = 0;

    let mut only1 = Vec::new();
    let mut only2 = Vec::new();
    let mut common = Vec::new();

    while i < v1.len() && j < v2.len() {

        let detection = detect_s(&v1[i], &v2[j]);

        if detection == Detection::Equale {
            common.push(v1[i].clone());
            i += 1;
            j += 1;

        } else if detection == Detection::Syounari {
            only1.push(v1[i].clone());
            i += 1;

        } else {
            only2.push(v2[j].clone());
            j += 1;
        }
    }

    // FILE1に残っている行
    while i < v1.len() {
        only1.push(v1[i].clone());
        i += 1;
    }

    // FILE2に残っている行
    while j < v2.len() {
        only2.push(v2[j].clone());
        j += 1;
    }

    (only1, only2, common)
}

fn merge_lines(
    only1: Vec<String>,
    only2: Vec<String>,
    common: Vec<String>,
) -> Vec<(String, usize)> {

    let mut lines = Vec::new();

    for s in only1 {
        lines.push((s, 1));
    }

    for s in only2 {
        lines.push((s, 2));
    }

    for s in common {
        lines.push((s, 3));
    }

    lines.sort_by(|a, b| a.0.cmp(&b.0));

    lines
}

fn main()->Result<(),()>{
    let args = Args::parse();
    //println!("{:?}",args);
    //
    let content_1= file_open(&args.file1).unwrap();
    let content_2 = file_open(&args.file2).unwrap();

    let (only1, only2, common) = make_lines(&content_1, &content_2);

    let lines = merge_lines(only1, only2, common);

    for (s, column) in lines {
        if column == 1 && !args.suppress_file1 {
            println!("{}", s);
        } else if column == 2 && !args.suppress_file2 {
            let tabs = if args.suppress_file1 { 0 } else { 1 };
            println!("{}{}", "\t".repeat(tabs), s);
        } else if column == 3 && !args.suppress_common {
            let tabs =
                (!args.suppress_file1 as usize) +
                (!args.suppress_file2 as usize);

            println!("{}{}", "\t".repeat(tabs), s);
        }
    }


    /*
    if !args.suppress_file1 {
        // only1 を表示
        for s in only1{
            println!("{}",s);
        }
    }

    if !args.suppress_file2 {
        // only2 を表示
        for s in only2{
            println!("{}",s);
        }
    }

    if !args.suppress_common {
        // common を表示
        for s in common{
            println!("{}",s);
        }
    }
     */



    Ok(())
}

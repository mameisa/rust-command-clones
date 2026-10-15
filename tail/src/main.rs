use clap::Parser;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::{Read, Write};

/// シンプルなCLIツールのサンプル
#[derive(Parser)]
struct Args {
    /// -c : Byte数指定
    #[arg(short)]
    count: Option<usize>,

    ///-n : 行数指定
    #[arg(short)]
    nlines: Option<usize>,

    ///-q : ファイル名表示
    #[arg(short)]
    quiet: bool,

    ///複数ファイル名
    //#[arg(long)]
    files: Vec<String>,
}

fn validate_args(args: &Args) -> Result<(), ()> {
    if (args.count.is_some() && args.nlines.is_some()) || args.files.is_empty() {
        return Err(());
    }
    Ok(())
}
#[derive(PartialEq)]
enum Mode {
    Byte(usize),
    Line(usize),
    All,
}

fn parse_args(tup: (Option<usize>, Option<usize>)) -> Result<Mode, ()> {
    match tup {
        (Some(c), None) => Ok(Mode::Byte(c)),
        (None, Some(n)) => Ok(Mode::Line(n)),
        (None, None) => Ok(Mode::All),
        _ => Err(()),
    }
}

fn readfiles(file_paths: &Vec<String>) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    // 指定されたファイルを順番に処理する
    let mut filemap: HashMap<String, Vec<u8>> = HashMap::new();

    for path in file_paths {
        let mut bytes = Vec::new();
        match File::open(&path) {
            Ok(mut file) => {
                let _ = file.read_to_end(&mut bytes)?;
                filemap.insert(path.to_string(), bytes);
            }
            Err(err) => {
                eprintln!("tail: {}: {}", path, err);
                std::process::exit(1);
            }
        }
    }
    Ok(filemap)
}

fn write_bytedis(
    b_writename: bool,              //ファイル名を標準出力する
    target_f: &String,              //対象のファイル名
    num_byte: usize,                //後ろから表示するバイト数
    map: &HashMap<String, Vec<u8>>, //引数解析でbuildした[ファイル名：Byte列]
) -> std::io::Result<()> {
    for (filename, v) in map {
        if filename == target_f {
            let mut stdout = io::stdout().lock();
            if b_writename {
                println!("==>{}<==", filename);
            }
            if num_byte > v.len() {
                stdout.write_all(v)?;
            } else {
                let v_slice = &v[(v.len() - num_byte)..];
                //println!("{:?}",v_slice);
                stdout.write_all(v_slice)?;
            }
        }
    }
    Ok(())
}

fn write_line_dis(
    b_writename: bool, //ファイル名を標準出力する
    target_f: &String, //対象のファイル名
    num_lines: usize,  //後ろから表示するライン数
) -> Result<(), Box<dyn std::error::Error>> {
    if num_lines != 0 {
        let file = File::open(target_f)?;
        let reader = BufReader::new(file);

        // 行を格納するリングバッファ（VecDeque）
        let mut lines_deque: VecDeque<String> = VecDeque::with_capacity(num_lines);

        for line in reader.lines() {
            let line = line?;
            if lines_deque.len() >= num_lines {
                lines_deque.pop_front(); // 最も古い行を削除
            }
            lines_deque.push_back(line);
        }

        if b_writename {
            println!("==>{}<==", target_f);
        }
        // 格納された行を順に出力
        for line in lines_deque {
            println!("{}", line);
        }
    } //if num_lines != 0
    Ok(())
}

fn write_line_all(
    b_writename: bool, //ファイル名を標準出力する
    target_f: &String, //対象のファイル名
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(target_f)?;
    let reader = BufReader::new(file);

    if b_writename {
        println!("==>{}<==", target_f)
    };
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

fn write_dis(
    b_writename: bool,
    mode: &Mode,
    fname: &String,
    filemap: &HashMap<String, Vec<u8>>,
) -> Result<(), Box<dyn Error>> {
    match mode {
        Mode::Byte(c) => {
            // 末尾 c byte,複数ファイルかの引数、quietフラグを渡す
            write_bytedis(b_writename, fname, *c, filemap)?;
            Ok(())
        }
        Mode::Line(n) => {
            // 末尾 n 行,複数ファイルかの引数、quietフラグを渡す

            write_line_dis(b_writename, fname, *n)
        }
        Mode::All => {
            // 全行,複数ファイルかの引数、quietフラグを渡す

            write_line_all(b_writename, fname)
        }
    } //match mode
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 引数をパースして構造体に格納
    let args = Args::parse();
    let result = validate_args(&args);

    //let filemap=readfiles(&args.files)?;

    match result {
        Err(_) => {
            eprintln!("invalid option");
            std::process::exit(1);
        }
        _ => {
            println!("ok! valid option");
            if let Ok(mode) = parse_args((args.count, args.nlines)) {
                let mut filemap = HashMap::new();
                if matches!(mode, Mode::Byte(_)) {
                    filemap = readfiles(&args.files)?;
                }
                for name in &args.files {
                    write_dis(args.files.len() > 1 && !args.quiet, &mode, name, &filemap)?
                } //for name in args.files                   
            } //if let Ok
        } //_=>
    } //match result
    Ok(())
}



//use std::io::Error;

//use std::env::args;

enum Mode {
    Byte,
    Char,
    Field,
}

struct Config {
    mode:Mode,
    delimiter: String,
    list:Vec<u32>,
    filename:String,
}


fn parse_args(config:&mut Config) -> Result<Mode,&'static str> {

    
    
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2{
        return Err("invalid option!");
    }

    let mode=match args[1].as_str() {
        "-b" => Ok(Mode::Byte),
        "-c" => Ok(Mode::Char),
        "-f" => Ok(Mode::Field),
        "-d" => {
            /*ここにdelimiter特有の引数解析コードを書く */
            if args.len() >= 3 && args[2] == ':'.to_string(){
                config.delimiter = true;
            }

            Ok(Mode::Field)
        }

        _ => {
            eprintln!("invalid option");
            return Err("???");
        }
    };//match

    mode
}//



fn main() {
    println!("Hello, world!");

    let mut config = Config{delimiter:false};

    let mode = parse_args(&mut config);

}

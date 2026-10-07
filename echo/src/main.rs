use std::env;

fn main() {

    let mut omit_newline = false;
    let mut texts = Vec::new();

    for arg in env::args().skip(1) {
        if arg == "-n" {
            omit_newline = true;
        } else {
            texts.push(arg);
        }
    }

    let output = texts.join(" ");

    if omit_newline {
        print!("{}", output);
    } else {
        println!("{}", output);
    }
}
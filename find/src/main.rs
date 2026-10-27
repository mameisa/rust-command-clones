use clap::Parser;
use std::fs;
use std::path::{Path,PathBuf};

#[derive(Debug)]
enum Expr {
    Leaf(Predicate),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

#[derive(Debug)]
enum Predicate {
    Name(String),
    Type(char),
}

#[derive(Parser,Debug)]
struct Args {
    ///
    path:String,
    ///
    #[arg(allow_hyphen_values = true)]
    expr:Vec<String>,
}


fn eval_predicate(pred: &Predicate, path: &Path) -> bool {
    match pred {
        Predicate::Name(pattern) => {
            path.file_name()
                .and_then(|s| s.to_str())
                .map(|name| {
                    if pattern.starts_with("*.") {
                        name.ends_with(&pattern[1..])
                    } else {
                        name == pattern
                    }
                })
                .unwrap_or(false)
        }
        Predicate::Type(t) => {
            let md = match fs::metadata(path) {
                Ok(m) => m,
                Err(_) => return false,
            };
            match *t {
                'f' => md.is_file(),
                'd' => md.is_dir(),
                _ => false,
            }
        }
    }
}

fn find_expr(path: &Path, expr: &Expr, results: &mut Vec<PathBuf>) {

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if eval_expr(expr, &path) {
            results.push(path.clone());
        }

        if path.is_dir() {
            find_expr(&path, expr, results);
        }
    }
}

fn eval_expr(expr: &Expr, path: &Path) -> bool {
    match expr {
        Expr::Leaf(pred) => eval_predicate(pred, path),
        Expr::And(l, r) => eval_expr(l, path) && eval_expr(r, path),
        Expr::Or(l, r) => eval_expr(l, path) || eval_expr(r, path),
        Expr::Not(e) => !eval_expr(e, path),
    }
}

struct ExprParser {
    tokens: Vec<String>,
    pos: usize,
}

impl ExprParser {

    fn next(&mut self) -> Option<String> {

        if self.pos >= self.tokens.len() {
            None
        } else {
            let s = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(s)
        }

    }

    fn is_primary_start(tok: &str) -> bool {
        matches!(tok, "-name" | "-type" | "!" | "(")
    }

    fn parse_or(&mut self) -> Expr {
        let mut expr = self.parse_and();
        while let Some(tok) = self.tokens.get(self.pos).map(|s| s.as_str()) {
            if tok == "-o" {
                self.next();
                let right = self.parse_and();
                expr = Expr::Or(Box::new(expr), Box::new(right));
            } else {
                break;
            }
        }
        expr
    }

    fn parse_and(&mut self) -> Expr {

        let mut expr = self.parse_primary();

            while let Some(tok) = self.tokens.get(self.pos).map(|s| s.as_str()) {
                if Self::is_primary_start(tok) {
                    let right = self.parse_primary();
                    expr = Expr::And(Box::new(expr), Box::new(right));
                } else {
                    break;
                }
            }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next().as_deref(){
            Some("-name") => {
                let pattern = self.next().unwrap();
                Expr::Leaf(Predicate::Name(pattern))
            }
            Some("-type") => {
                let ty = self.next().unwrap();
                Expr::Leaf(Predicate::Type(ty.chars().next().unwrap()))
            }
            Some("!") => {
                let e = self.parse_primary();
                Expr::Not(Box::new(e))
            }
            Some("(") => {
                let e = self.parse_or();
                match self.next().as_deref() {
                    Some(")") => e,
                    _ => panic!("')' がありません"),
                }
            }
            _ => panic!("unexpected token"),
        }//match self.next()
    }

    fn parse(&mut self) -> Expr {
        let expr = self.parse_or();

        if self.pos < self.tokens.len() {
            panic!("unexpected token: {}", self.tokens[self.pos]);
        }

        expr
    }
}



fn main() {

    let args = Args::parse();

    let root = Path::new(&args.path);

    let mut parser = ExprParser {
        tokens: args.expr,
        pos: 0,
    };

    let expr = parser.parse();

    let mut results = Vec::new();
    find_expr(root, &expr, &mut results);

    for path in results {
        println!("{}", path.display());
    }
}

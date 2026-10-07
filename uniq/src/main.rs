

#[derive(Debug)]
struct Config {
    count: bool,
    input: Option<String>,
    output: Option<String>,
}

fn open_input(path: &Option<String>) -> Result<Box<dyn std::io::BufRead>, String> {
    match path {
        Some(p) => {
            let file = std::fs::File::open(p).map_err(|e| format!("Failed to open input file {}: {}", p, e))?;
            Ok(Box::new(std::io::BufReader::new(file)))
        }
        None => Ok(Box::new(std::io::BufReader::new(std::io::stdin()))),
    }
}


fn open_output(path: &Option<String>) -> Result<Box<dyn std::io::Write>, String> {
    match path {
        Some(p) => {
            let file = std::fs::File::create(p).map_err(|e| format!("Failed to create output file {}: {}", p, e))?;
            Ok(Box::new(file))
        }
        None => Ok(Box::new(std::io::stdout())),
    }
}

fn uniq<R: std::io::BufRead, W: std::io::Write>(
    reader: R,
    mut writer: W,
    count: bool,
) -> Result<(), String> {

    let mut prev_line: Option<String> = None;
    let mut count_lines: usize = 0;

    for line_result in reader.lines() {

        let line =
            line_result.map_err(|e| format!("Failed to read line: {}", e))?;

        match prev_line.as_ref() {

            Some(previous) if previous == &line => {
                count_lines += 1;
            }

            Some(previous) => {
                if count {
                    writeln!(writer, "{} {}", count_lines, previous)
                        .map_err(|e| format!("Failed to write line: {}", e))?;
                } else {
                    writeln!(writer, "{}", previous)
                        .map_err(|e| format!("Failed to write line: {}", e))?;
            }
                prev_line = Some(line);
                count_lines = 1;
            }

            None => {
                prev_line = Some(line);
                count_lines = 1;
            }
        }
    }

    if let Some(previous) = prev_line.as_ref() {
        if count {
            writeln!(writer, "{} {}", count_lines, previous)
                .map_err(|e| format!("Failed to write line: {}", e))?;
        } else {
            writeln!(writer, "{}", previous)
                .map_err(|e| format!("Failed to write line: {}", e))?;
        }
    }

    Ok(())
}

fn run(config: &Config) -> Result<(), String> {

    let reader = open_input(&config.input)?;
    let writer = open_output(&config.output)?;
    
    uniq(reader, writer, config.count)?;
    
    Ok(())
}

fn parse_args() -> Result<Config, String> {

    let args: Vec<String> = std::env::args().collect();

    let mut config = Config {
        count: false,
        input: None,
        output: None,
    };



    let mut i = 1;
  

    while i < args.len() {

        match args[i].as_str() {
            "-c" | "--count" => {
                config.count = true;
                i += 1;
            }
            _ => {
                
            }
        }

        if i < args.len() && !args[i].as_str().is_empty(){
                config.input = Some(args[i].clone());
                i += 1;
        }      
        if i < args.len() {
            config.output = Some(args[i].clone());
            i += 1;
        }

    }

    Ok(config)

}

fn main() {
    let config = parse_args().unwrap();
    run(&config).unwrap();
}

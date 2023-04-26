use std::{
    env,
    fs::File,
    io::stdin,
    io::{BufRead, Read},
    process,
};

use roxy::token::Token;


struct Scanner {}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {}
    }

    fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }

    fn error(&self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: u32, where_: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_, message);
    }
}


fn run_file(path: &str) -> Result<(), anyhow::Error>{
    let mut f = File::open(path).expect("File not found");
    let mut source = String::new();

    f.read_to_string(&mut source).expect("Failed to read file");

    run(&source)
}

fn run_prompt() -> Result<(), anyhow::Error>{
    let mut handler = stdin().lock();
    loop {
        let mut source = String::new();
        if handler.read_line(&mut source).is_err() || source.is_empty() {
            break;
        }
        match run(&source) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e)
        };
    }
    Ok(())
}

fn run(source: &str) -> Result<(), anyhow::Error>{
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())

}

fn main() -> anyhow::Result<()> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() > 1 {
        println!("Usage: roxy [script]");
        process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

use std::{
    env,
    fs::File,
    io::stdin,
    io::{self, Write},
    io::{BufRead, Read},
    process,
};

use roxy::interpreter::Interpreter;
use roxy::parser::Parser;
use roxy::scanner::Scanner;

fn run_file(path: &str) -> Result<(), anyhow::Error> {
    let mut f = File::open(path).expect("File not found");
    let mut source = String::new();

    f.read_to_string(&mut source).expect("Failed to read file");

    run(&source)
}

fn run_prompt() -> Result<(), anyhow::Error> {
    let mut handler = stdin().lock();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut source = String::new();
        if handler.read_line(&mut source).is_err() || source.is_empty() {
            break;
        }
        match run(&source) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        };
    }
    Ok(())
}

fn run(source: &str) -> Result<(), anyhow::Error> {
    let mut scanner = Scanner::new(source);
    let mut interpreter = Interpreter::new();
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expr = parser
        .parse()
        .ok_or(anyhow::anyhow!("Failed to parse expression"))?;
    let out = interpreter.interpret(expr)?;
    println!("{:?}", out);
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

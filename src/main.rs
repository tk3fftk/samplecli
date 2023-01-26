use anyhow::{bail, ensure, Context, Result};

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invaid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invaid syntax at {}", pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            if self.0 {
                println!("t: {:?}, s:{:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "tk3fftk",
    about = "Super awesome sample RPN calculator", 
    long_about=None
)]
struct Opts {
    // Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[arg(name = "FILE")]
    formula_file: Option<String>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        // ロックすることでバッファリングして読み出せるStdinLock型が使えて高速になる
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        let v = vec![
            ("5", 5),
            ("50", 50),
            ("-50", -50),
            ("2 3 +", 5),
            ("2 3 *", 6),
            ("2 3 -", -1),
            ("2 3 /", 0),
            ("2 3 %", 2),
        ];
        for t in v {
            assert_eq!(calc.eval(t.0).unwrap(), t.1);
        }
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        let v = vec!["", "1 1 1 +", "+ 1 1"];
        for t in v {
            assert!(calc.eval(t).is_err());
        }
    }
}

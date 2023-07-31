mod interpreter;
mod repl;

use eyre::Result;

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    match args.len() {
        0 => repl(),
        1 => execute(&args[0]),
        _ => Err(eyre::eyre!("Usage: rlox [path]")),
    }
}

fn repl() -> Result<()> {
    // repl::Repl::new().run()
    interpreter::Interpreter::new().execute("1 + 2 * 3")
}

fn execute(path_to_code: &str) -> Result<()> {
    let code = std::fs::read_to_string(path_to_code)?;
    interpreter::Interpreter::new().execute(&code)
}

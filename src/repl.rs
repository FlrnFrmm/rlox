use eyre::Result;

pub struct Repl {}

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Result<()> {
        println!("Running REPL");
        Ok(())
    }
}

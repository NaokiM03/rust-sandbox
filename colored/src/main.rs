use colored::*;
use rustyline::{error::ReadlineError, Editor};


fn main() {
    const PROMPT: &str = ">> ";
    let mut editor = Editor::<()>::new();

    loop {
        println!("{}", &PROMPT.red()); // ok
        let readline = editor.readline(&PROMPT.red()); // ng
        match readline {
            Ok(line) => {
                println!("Input: {}", line);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

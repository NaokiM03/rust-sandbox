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
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

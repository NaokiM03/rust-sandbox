use clap::{App, Arg, SubCommand, crate_name};

fn main() {
    let app = App::new(crate_name!())
        .arg(Arg::from_usage("<fa> 'first argument'"))
        .arg(Arg::from_usage("[flg] -f --flag 'first flag'"))
        .arg(Arg::from_usage("-o --option [OPT] 'option'"))
        .subcommand(SubCommand::with_name("sub")
            .about("subcommand") 
            .arg(Arg::from_usage("[subflg] -f --flag 'sub command flag'"))
        );
    
    let matches = app.get_matches();

    if let Some(o) = matches.value_of("fa") {
        println!("Value for fa: {}", o);
    }

    if let Some(o) = matches.value_of("option") {
        println!("Value for option: {}", o);
    }

    println!("flg is {}", if matches.is_present("flg") {"ON"} else {"OFF"});

    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub");
        println!("subflg is {}", if matches.is_present("subflg") {"ON"} else {"OFF"});
    }
}

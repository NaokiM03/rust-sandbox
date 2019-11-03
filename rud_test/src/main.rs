use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;
use std::process::exit;
use std::path::Path;

use std::process::{Command, Stdio};

use path_slash::PathExt;


fn usage() {
    eprintln!("引数はコンパイルするファイル名を指定してください");
    exit(1)
}

fn read_file(filename: &str) -> String {
    let mut input = String::new();
    let mut fp = File::open(filename).expect("ファイルが開けませんでした");
    fp.read_to_string(&mut input).expect("ファイル読み込み中にエラーが発生しました");
    return input
}

fn is_main_function(token: &str) -> bool {
    token == "main()"
}

fn is_main_function_exist(tokens: Vec<&str>) -> bool {
    tokens.iter().any(|&x| is_main_function(x))
}

fn tokenize(code: &str) -> Vec<&str> {
    code.split_whitespace().collect()
}

fn echo_into_new_file(code: &str, path: &Path) -> io::Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(code.as_bytes())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }

    let first_arg: String = args[1].clone();
    let filepath: String = Path::new(&first_arg).to_slash().unwrap();

    let code: String = read_file(&filepath);
    let tokens: Vec<&str> = tokenize(&code);

    let split_filepath: Vec<&str> = filepath.split("/").collect();
    let filename: String = split_filepath[2].to_string();
    let split_filename: Vec<&str> = filename.split(".").collect();
    let directory_name: String = split_filename[0].to_string();

    fs::create_dir("./examples/".to_string() + &directory_name);
    fs::create_dir("./examples/".to_string() + &directory_name + &"/src");

    echo_into_new_file("[package]
name = \"rud_test\"
version = \"0.1.0\"
authors = [\"NaokiM03 <mattsun.phone@gmail.com>\"]
edition = \"2018\"

[dependencies]
", &Path::new(&("./examples/".to_string() + &directory_name + "/Cargo.toml"))).unwrap();

    echo_into_new_file("/target
**/*.rs.bk
", &Path::new(&("./examples/".to_string() + &directory_name + "/.gitignore"))).unwrap();


    let mut file = File::create("./examples/hello_world/src/main.rs").unwrap();

    let is_main_function_exist: bool = is_main_function_exist(tokens);
    if !is_main_function_exist {
        file.write_fmt(format_args!("fn main() {{\n")).unwrap();
    }
    file.write_fmt(format_args!("    println!(\"Hello, world!\");\n")).unwrap();
    if !is_main_function_exist {
        file.write_fmt(format_args!("}}\n")).unwrap();
    }

    let mut process = Command::new("cargo")
        .current_dir("./examples/hello_world")
        .arg("run")
        .stdin(Stdio::null())
        .spawn().expect("failed to run");
    process.wait();
}

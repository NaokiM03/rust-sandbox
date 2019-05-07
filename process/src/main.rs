fn main() {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };

    let hello = String::from_utf8(output.stdout.to_vec()).unwrap();

    println!("{}", hello);
}

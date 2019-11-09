
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GET https://www.rust-lang.org/");

    let mut res = reqwest::get("https://www.rust-lang.org/")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    std::io::copy(&mut res, &mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}

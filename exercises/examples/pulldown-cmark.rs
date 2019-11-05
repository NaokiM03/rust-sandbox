use pulldown_cmark as Cmark;

fn main() {
    let input = "Hello world, this is a ~~complicated~~ *very simple* example.";

    let mut options = Cmark::Options::empty();
    options.insert(Cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = Cmark::Parser::new_ext(input, options);

    let mut html_output = String::with_capacity(input.len() * 3 / 2);
    Cmark::html::push_html(&mut html_output, parser);

    println!("Out: {}", &html_output);
}

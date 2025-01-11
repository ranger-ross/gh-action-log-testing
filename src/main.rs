use std::io::{self, IsTerminal, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    println!("stdout = {}", std::io::stdout().is_terminal());
    println!("stderr = {}", std::io::stderr().is_terminal());

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    writeln!(&mut stdout, "green text!").unwrap();
}

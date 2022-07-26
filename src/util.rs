use std::io::Write;
use termcolor::{StandardStream, ColorSpec, Color, WriteColor};

pub fn print_error(msg: String) {
	let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
	write!(&mut stdout, "error: ").unwrap();
	stdout.reset().unwrap();
	write!(&mut stdout, "{}", msg).unwrap();
}

pub fn print_success(msg: String) {
	let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Auto);
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap();
	write!(&mut stdout, "success: ").unwrap();
	stdout.reset().unwrap();
	write!(&mut stdout, "{}", msg).unwrap();
}

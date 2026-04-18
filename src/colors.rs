use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use std::io::Write;

/// Print a colored error message
pub fn error(msg: &str) {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    if stderr.supports_color() {
        let _ = stderr.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
        let _ = writeln!(stderr, "{}", msg);
        let _ = stderr.reset();
    } else {
        eprintln!("{}", msg);
    }
}

/// Print a colored warning message
pub fn warning(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    if stdout.supports_color() {
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}

/// Print a colored success message
pub fn success(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    if stdout.supports_color() {
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}

/// Print a colored info message
pub fn info(msg: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    if stdout.supports_color() {
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Cyan)));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}

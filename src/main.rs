use ansi_term::{
    ANSIByteStrings, ANSIString, ANSIStrings,
    Colour::{Blue, Cyan, Green, Red, Yellow},
    Style,
};

fn main() {
    println!("This is in red: {}", Red.paint("a red string"));
    println!(
        "How about some {} and {}?",
        Style::new().bold().paint("bold"),
        Style::new().underline().paint("underline")
    );
    println!(
        "Demonstrating {} and {}!",
        Blue.bold().paint("blue bold"),
        Yellow.underline().paint("yellow underline")
    );

    println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));

    println!(
        "Yellow on blue: {}",
        Style::new().on(Blue).fg(Yellow).paint("yow!")
    );
    println!(
        "Also yellow on blue: {}",
        Cyan.on(Blue).fg(Yellow).paint("zow!")
    );

    println!("******");

    let some_value = format!("{:b}", 42);
    let strings: &[ANSIString<'static>] =
        &[Red.paint("["), Red.bold().paint(some_value), Red.paint("]")];

    println!("Value: {}", ANSIStrings(strings));

    Green
        .paint("user data".as_bytes())
        .write_to(&mut std::io::stdout())
        .unwrap();

    ANSIByteStrings(&[
        Green.paint("user data 1\n".as_bytes()),
        Green.bold().paint("user data 2\n".as_bytes()),
    ])
    .write_to(&mut std::io::stdout())
    .unwrap();
}

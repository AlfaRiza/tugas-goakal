use std::io::{self, Write};

pub fn read_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn print_output(pal: &str) {
    println!("Palindrome terpanjang: \"{}\"\n", pal);
}

use crate::io_handler::{read_input, print_output};
use crate::palindrome::longest_palindrome;

pub fn run() {
    loop {
        let input = read_input("Masukkan kata/kalimat (ketik 'exit' untuk keluar): ");

        if input.eq_ignore_ascii_case("exit") {
            println!("Program selesai.");
            break;
        }

        if input.is_empty() {
            println!("Input kosong!\n");
            continue;
        }

        let palindrome = longest_palindrome(&input);
        print_output(&palindrome);
    }
}

use std::io::{self, Write};

fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }

    // Manacher transform
    let t_len = 2 * n + 1;
    let mut t: Vec<char> = Vec::with_capacity(t_len);
    for i in 0..t_len {
        if i % 2 == 0 {
            t.push('|');
        } else {
            t.push(chars[(i - 1) / 2]);
        }
    }

    let mut p = vec![0usize; t_len];
    let mut center = 0usize;
    let mut right = 0usize;

    for i in 0..t_len {
        if i < right {
            let mir = 2 * center - i;
            p[i] = std::cmp::min(p[mir], right - i);
        }

        // expand
        while i + 1 + p[i] < t_len &&
              i >= 1 + p[i] &&
              t[i + 1 + p[i]] == t[i - 1 - p[i]] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    // find result
    let (mut best_center, mut best_len) = (0usize, 0usize);
    for i in 0..t_len {
        if p[i] > best_len {
            best_len = p[i];
            best_center = i;
        }
    }

    let start = (best_center - best_len) / 2;
    chars[start..start + best_len].iter().collect()
}

fn main() {
    loop {
        print!("Masukkan kata/kalimat (pisahkan dengan space) (ketik 'exit' untuk keluar): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Jika pengguna mengetik "exit" (case-insensitive untuk ASCII),
        // tampilkan pesan dan hentikan loop utama sehingga program selesai.
        if input.eq_ignore_ascii_case("exit") {
            println!("Program selesai.");
            break;
        }

        // Jika input kosong setelah `trim()`, lewati iterasi ini
        // untuk menghindari pemanggilan `longest_palindrome` pada input tidak valid.
        if input.is_empty() {
            println!("Input kosong!\n");
            continue;
        }

        let palindrome = longest_palindrome(input);

        println!("Palindrome terpanjang: \"{}\"\n", palindrome);
    }
}

pub fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return String::new();
    }

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

        while i + 1 + p[i] < t_len &&
              i >= 1 + p[i] &&
              t[i + 1 + p[i]] == t[i - 1 - p[i]]
        {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

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

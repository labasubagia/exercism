pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .try_fold((0, 0), |(sum, count), c| {
            if c.is_whitespace() {
                return Some((sum, count));
            }
            c.to_digit(10)
                .map(|n| if count % 2 == 1 { n * 2 } else { n })
                .map(|n| if n > 9 { n - 9 } else { n })
                .map(|n| (sum + n, count + 1))
        })
        .map_or(false, |(sum, count)| count >= 2 && sum % 2 == 0)
}

pub fn is_valid2(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10)
                .map(|n| if count % 2 == 1 { n * 2 } else { n })
                .map(|n| if n > 9 { n - 9 } else { n })
                .map(|n| (sum + n, count + 1))
        })
        .map_or(false, |(sum, count)| count >= 2 && sum % 2 == 0)
}

use std::collections::HashMap;

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut map = HashMap::<char, usize>::new();
    input.iter().for_each(|&word| {
        word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .for_each(|c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            })
    });
    map
}

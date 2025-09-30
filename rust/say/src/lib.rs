use std::collections::HashMap;

fn base() -> HashMap<u64, String> {
    let mut map = HashMap::new();

    for (i, &word) in [
        "", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ].iter().enumerate() {
        map.insert(i as u64, word.to_string());
    }

    for (i, &word) in ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"].iter().enumerate() {
        map.insert((20 + i * 10) as u64, word.to_string());
    }

    for i in 21..100 {
        if i % 10 != 0 {
            let word = format!("{}-{}", map[&(i / 10 * 10)], map[&(i % 10)]);
            map.insert(i, word);
        }
    }

    for i in 100..1000 {
        let word = if i % 100 == 0 {
            format!("{} hundred", map[&(i / 100)])
        } else {
            format!("{} hundred {}", map[&(i / 100)], map[&(i % 100)])
        };
        map.insert(i, word);
    }

    map
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let base = base();
    let mut parts = Vec::new();

    for &scale in ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"].iter() {
        if n % 1000 != 0 {
            let mut chunk = base[&(n % 1000)].clone();
            if !scale.is_empty() {
                chunk.push(' ');
                chunk.push_str(scale);
            }
            parts.push(chunk);
        }
        n /= 1000;
        if n == 0 {
            break;
        }
    }

    parts.reverse();
    parts.join(" ")
}

use std::fmt::Display;

pub struct Luhn {
    valid: bool
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        let s = input.to_string();
        if s.find(|c: char| !c.is_whitespace() && !c.is_numeric()).is_some() {
            return Luhn{valid: false};
        }

        let mut d: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
        if d.len() <= 1 {
            return Luhn{valid: false};
        }

        let mut total = 0;
        let mut multiplier = 1;
        while let Some(digit) = d.pop() {
            let mut sum = multiplier * digit;
            if sum > 9 {
                sum -= 9;
            }
            total += sum;
            multiplier = 3 - multiplier;
        }
        Luhn{valid: total % 10 == 0}
    }
}
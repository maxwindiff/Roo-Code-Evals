use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Default)]
pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.insert(student.to_string()) {
            self.roster.entry(grade).or_default().insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.get(&grade).map_or_else(Vec::new, |s| s.iter().cloned().collect())
    }
}

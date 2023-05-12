use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
pub struct Word(pub String);

impl Hash for Word {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Default)]
pub struct WordStatistics {
    statistics: HashMap<Word, usize>,
}
impl WordStatistics {
    pub fn add_line(&mut self, line: &str) {
        log::debug!("tokenizing line");
        for token in line.split(" ") {
            let word = Word(token.to_string());
            match self.statistics.entry(word) {
                Entry::Occupied(mut e) => e.insert(*e.get() + 1),
                Entry::Vacant(e) => *e.insert(1),
            };
        }
    }

    pub fn top_n(&self, n: usize) -> Vec<(Word, usize)> {
        let mut all: Vec<(Word, usize)> = self.statistics
            .iter()
            .map(|(word, num)| (word.clone(), num.clone()))
            .collect();
        all.sort_unstable_by_key(|(_, num)| *num);

        all[0..n].into()
    }
}


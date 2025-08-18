use indicatif::ParallelProgressIterator;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::collections::HashSet;
use std::iter::zip;

lazy_static! {
    static ref wordle_words: Vec<String> = include_str!("original_wordle_words.txt")
        .split_ascii_whitespace()
        .filter(|word| word.len() == 5)
        .map(|s| s.to_string())
        .collect();
}
fn main() {
    println!("Total word count: {}", wordle_words.len());

    let best_word = wordle_words
        .clone()
        .into_par_iter()
        .progress()
        .min_by_key(|word| score(word))
        .unwrap();

    println!("best word is ({})", best_word);
}

fn score(starting_word: &str) -> u64 {
    let mut score: u64 = 0;
    for correct_answer in wordle_words.iter() {
        if let CheckResult::Incorrect(hints) = check(starting_word, correct_answer) {
            for second_guess in wordle_words.iter() {
                if hints.iter().all(|&hint| second_guess.is_valid_for(hint)) {
                    score += 1;
                }
            }
        }
    }
    score
}

enum CheckResult {
    Correct,
    Incorrect(HashSet<Hint>),
}

fn check(guess: &str, answer: &str) -> CheckResult {
    let mut hints: HashSet<Hint> = HashSet::new();
    if guess == answer {
        return CheckResult::Correct;
    }
    for (idx, (gc, ac)) in zip(guess.chars(), answer.chars()).enumerate() {
        if gc == ac {
            hints.insert(Hint::ContainsAt(gc, idx));
            hints.insert(Hint::Contains(gc));
        } else if answer.chars().any(|c| c == gc) {
            hints.insert(Hint::Contains(gc));
            hints.insert(Hint::DoesNotContainAt(gc, idx));
        } else {
            hints.insert(Hint::DoesNotContain(gc));
        }

    }
    CheckResult::Incorrect(hints)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Hint {
    DoesNotContain(char),
    DoesNotContainAt(char, usize),
    Contains(char),
    ContainsAt(char, usize),
}

trait Match {
    fn is_valid_for(&self, hint: Hint) -> bool;
}

impl Match for String {
    fn is_valid_for(&self, hint: Hint) -> bool {
        match hint {
            Hint::DoesNotContain(c) => !self.chars().any(|oc| c == oc),
            Hint::DoesNotContainAt(c, idx) => self.chars().nth(idx) != Some(c),
            Hint::Contains(c) => self.chars().any(|oc| c == oc),
            Hint::ContainsAt(c, idx) => self.chars().nth(idx) == Some(c),
        }
    }
}

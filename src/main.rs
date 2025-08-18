use indicatif::ParallelProgressIterator;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::collections::HashSet;
use std::iter::zip;

lazy_static! {
    static ref wordle_words: Vec<Vec<char>> = include_str!("original_wordle_words.txt")
        .split_ascii_whitespace()
        .filter(|word| word.len() == 5)
        .map(|s| s.chars().collect())
        .collect();

    static ref letter_map: Vec<[bool; 26]> = wordle_words
        .iter()
        .map(|word| make_letter_map(word))
        .collect();
}

fn make_letter_map(word: &Vec<char>) -> [bool; 26] {
    let mut result: [bool; 26] = [false; 26];

    for &c in word{
        result[to_idx(c)] = true;
    }

    result
}

fn to_idx(c: char) -> usize {
    c as usize - ('a' as usize)
}

fn main() {
    println!("Total word count: {}", wordle_words.len());

    let best_word = (0..wordle_words.len())
        .into_par_iter()
        .progress()
        .min_by_key(|&word_idx| score(word_idx))
        .unwrap();

    println!("best word is ({})", best_word);
}

fn score(word_idx: usize) -> u64 {
    let mut score: u64 = 0;
    for correct_answer in wordle_words.iter() {
        if let CheckResult::Incorrect(hints) = check(word_idx, correct_answer) {
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

fn check(word_idx: usize, answer: &Vec<char>) -> CheckResult {
    let mut hints: HashSet<Hint> = HashSet::new();
    let guess = &wordle_words[word_idx];
    if guess == answer {
        return CheckResult::Correct;
    }
    for (idx, (&gc, &ac)) in zip(guess, answer).enumerate() {
        if gc == ac {
            hints.insert(Hint::ContainsAt(gc, idx));
            hints.insert(Hint::Contains(gc));
        } else if letter_map[word_idx][to_idx(gc)] {
            hints.insert(Hint::Contains(gc));
            hints.insert(Hint::DoesNotContainAt(gc, idx));
        } else {
            hints.insert(Hint::DoesNotContain(gc));
        }
    }
    // println!("Guess ({}) Answer({}) hints ({:?})", guess, answer, hints);
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

impl Match for Vec<char> {
    fn is_valid_for(&self, hint: Hint) -> bool {
        match hint {
            Hint::DoesNotContain(c) => !self.iter().any(|&oc| c == oc),
            Hint::DoesNotContainAt(c, idx) => self[idx] != c,
            Hint::Contains(c) => self.iter().any(|&oc| c == oc),
            Hint::ContainsAt(c, idx) => self[idx] == c,
        }
    }
}

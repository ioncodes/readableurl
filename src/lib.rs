extern crate rand;

use std::str;
use rand::seq::SliceRandom;

const ADJECTIVES: &'static [u8] = include_bytes!("./adjectives.txt");
const NOUNS: &'static [u8] = include_bytes!("./nouns.txt");

pub fn create(amount: usize, camelcase: bool) -> String {
    let adjectives: Vec<&str> = str::from_utf8(&ADJECTIVES).unwrap().split("\n").collect();
    let nouns: Vec<&str> = str::from_utf8(&NOUNS).unwrap().split("\n").collect();
    let mut url = String::from("");
    for _ in 0..amount - 1 {
        let mut adjective = adjectives.choose(&mut rand::thread_rng()).unwrap();
        url.push_str(adjective);
    }
    let mut noun = nouns.choose(&mut rand::thread_rng()).unwrap();
    url.push_str(noun);
    url
}
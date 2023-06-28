use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashSet;

trait Intersec {
    fn intersec_hash_set(&mut self, number: usize) -> Option<char>;
}

impl<'a, U: Iterator<Item = &'a str>> Intersec for U {
    fn intersec_hash_set(&mut self, number: usize) -> Option<char> {
        let mut temp: HashSet<char> = self.next()?.chars().collect();
        for _i in 1..number {
            let comp: HashSet<char> = self.next()?.chars().collect();
            temp = temp.intersection(&comp).copied().collect();
        }
        temp.into_iter().next()
    }
}

fn main() {
    println!("Day 3");

    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {display}: {why}")
    }

    let mut solution = 0;
    let rucksack: Vec<&str> = s.split('\n').collect();

    // Part 2
    let mut rucksack_iter = rucksack.into_iter();
    while rucksack_iter.len() >= 3 {
        if let Some(char) = rucksack_iter.intersec_hash_set(3) {
            if char.is_lowercase() {
                solution += char as u32 - 'a' as u32 + 1;
            } else {
                solution += char as u32 - 'A' as u32 + 26 + 1;
            }
        };
    }

    // Part 1
    // for bag in rucksack.iter() {
    // let bg = *bag;
    // let (left, right) = bg.split_at(bg.len() / 2);
    // let left_set: HashSet<char> = left.chars().into_iter().collect();
    // let right_set: HashSet<char> = right.chars().into_iter().collect();
    // let mut same = left_set.intersection(&right_set);

    // println!(
    //     "Left {:?}, Right {:?}, Differ {:?}",
    //     left_set, right_set, same
    // );

    // if let Some(char) = same.next() {
    //     if char.is_lowercase() {
    //         solution += *char as u32 - 'a' as u32 + 1;
    //     } else {
    //         solution += *char as u32 - 'A' as u32 + 26 + 1;
    //     }
    // };
    //}

    println!("Solution {solution}");
}

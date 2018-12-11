use std::fs;
use std::collections::HashMap;

trait TrueGroupBy : Iterator {
    fn true_group_by<K: std::hash::Hash + std::cmp::Eq, F>(self, to_key: F) -> HashMap<K, Vec<Self::Item>>
        where Self: Sized,
              F: Fn(&Self::Item) -> K,
              K: PartialEq,
    {
        let mut map: HashMap<K, Vec<Self::Item>> = HashMap::new();

        self.for_each(|item| {
            let key = (to_key)(&item);
            map.entry(key).or_insert_with(|| Vec::new()).push(item);
        });
        map
    }
}

impl<I: Iterator> TrueGroupBy for I {}

struct TwoThree {
    two: u32,
    three: u32,
}

impl TwoThree {
    fn checksum(&self) -> u32 {
        self.two * self.three
    }
}

impl std::ops::Add for TwoThree {
    type Output = TwoThree;

    fn add(self, other: TwoThree) -> TwoThree {
        TwoThree {
            two: self.two + other.two,
            three: self.three + other.three
        }
    }
}

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-02\\input.txt")
        .expect("peut");

    let zero = TwoThree{
        two: 0,
        three: 0,
    };
    let result = contents.lines()
        .map(|s| two_three(s))
        .fold(zero,|a, b| a + b);

    println!("part one {}", result.checksum());

    print!("part two ");
    match part_two(&contents.lines().collect()) {
        Some(result) => println!(
            "{} - {} = {}",
            result.0,
            result.1,
            same_chars(&result.0, &result.1)),
        None => println!("none")
    }
}

fn two_three(s: &str) -> TwoThree {
    let mut two = false;
    let mut three = false;
    for (_key, group)
            in &s.chars().into_iter().true_group_by(|x| *x) {
        let count = group.len();
        two |= count == 2;
        three |= count == 3;

        if two && three {
            break;
        }
    };

    TwoThree {
        two: two as u32,
        three: three as u32
    }
}

fn part_two(lines: &Vec<&str>) -> Option<(String, String)> {
    lines.iter().find_map(|line| {
        match lines.iter().find(|line2| off_by_one(line, line2)) {
            Some(b) => Some((String::from(*line), String::from(*b))),
            None => None
        }
    })
}

fn off_by_one(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    return a.chars().zip(b.chars())
        .filter(|ab| ab.0 == ab.1)
        .count()
        == a.len() - 1;
}

fn same_chars(a: &String, b: &String) -> String {
    let mut result = String::from("");

    for i in 0..a.len() {
        if i >= b.len() {
            break;
        }

        let ca = a.chars().nth(i).unwrap();
        let cb = b.chars().nth(i).unwrap();
        if ca == cb {
            result.push(ca);
        }
    }
    result
}
use std::fs::{read_to_string, File};

#[derive(Debug)]
struct Card<'a> {
    number: &'a str,
    winning_nums: String,
    card_nums: String,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let cards: Vec<Card> = lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(['|', ':']);
            Card {
                number: split.next().unwrap(),
                winning_nums: split.next().unwrap().trim().replace("  ", " "), //.split(',').collect::<String>().split(' ').map(|x| -> i32{x.parse().expect("faild")}).collect(),
                card_nums: split.next().unwrap().trim().replace("  ", " "), //.split(',').collect::<String>().split(' ').map(|x| -> i32{x.parse().unwrap()}).collect(),
            }
        })
        .collect();
    let mut score = 0;
    for card in cards {
        let winning: Vec<i32> = card
            .winning_nums
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mine: Vec<i32> = card
            .card_nums
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut cmatches = 0;
        winning.iter().for_each(|n| {
            if mine.contains(n) {
                cmatches += 1;
            }
        });

        score += match cmatches {
            0 => 0,
            1 => 1,
            n @ _ => 2_i32.pow(n - 1),
        };
        println!(
            "{} {:?} \n {:?} matches: {} current score: {}",
            card.number, winning, mine, cmatches, score
        );
    }
    println!("{}", score)
}

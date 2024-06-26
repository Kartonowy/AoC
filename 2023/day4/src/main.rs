use std::fs::{copy, read_to_string};

#[derive(Debug, Clone)]
struct Card<'a> {
    number: &'a str,
    winning_nums: String,
    card_nums: String,
    copies: i32,
}

impl<'a> Card<'a> {
    fn add_copy(&mut self) {
        self.copies += 1;
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut cards: Vec<Card> = lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(['|', ':']);
            Card {
                number: split.next().unwrap(),
                winning_nums: split.next().unwrap().trim().replace("  ", " "),
                card_nums: split.next().unwrap().trim().replace("  ", " "),
                copies: 1,
            }
        })
        .collect();
    let mut copymap: [i32; 198] = [1; 198];
    let mut index = 0;
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

        // part 2 start
        index += 1;
        for n in index..=index + cmatches - 1 {
            copymap[n] += 1;
        }

        println!(
            "{} {:?} \n {:?} matches: {}, {}",
            card.number, winning, mine, cmatches, index
        );
    }
    let sum: i32 = copymap.iter().sum();
    println!("{:?}", copymap);
    println!("{:?}", sum)
}

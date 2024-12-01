use std::{collections::HashMap, fs::read_to_string, io};
fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?
        .split("\n")
        .map(|e| e.to_string())
        .filter(|e| e != "")
        .collect::<Vec<String>>();

    let mut left = vec![];
    let mut right = vec![];
    for line in &input {
        let split: Vec<_> = line
            .splitn(2, "   ").collect();
        left.push(split[0].parse::<u32>().unwrap());
        right.push(split[1].parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let zipped = left.iter()
        .zip(right.iter())
        .collect::<Vec<(&u32, &u32)>>();

    let mut sum = 0;

    for zip in zipped {
        sum += zip.0.abs_diff(*zip.1);
    }


    println!("part one: {:?}", sum);

    let mut rightmap: HashMap<u32, u32> = HashMap::new();

    for numb in right {
        rightmap.entry(numb).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut similarity_score = 0;

    for numb in left {
        similarity_score += rightmap.get(&numb).unwrap_or(&0) * numb;
    }

    println!("part two: {:?}", similarity_score);

    Ok(())
}

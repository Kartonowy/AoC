use std::{fs::read_to_string, io};

use regex::Regex;

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;

    let re = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let captures = re.captures_iter(&input);



    let mut sum = 0;
    for cap in captures {
        let a = &cap[0].split(",").collect::<Vec<&str>>();
        let left = a[0].replace("mul(", "").parse::<u32>().unwrap();
        let right = a[1].replace(")", "").parse::<u32>().unwrap();
        sum += left * right;

    }
    println!("part one: {}", sum);


    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d*,\d*\))").unwrap();
    let captures = re.captures_iter(&input);

    let mut sum = 0;
    let mut allowed = true;
    for cap in captures {
        if cap[0].starts_with("m") && allowed {
            let a = &cap[0].split(",").collect::<Vec<&str>>();
            let left = a[0].replace("mul(", "").parse::<u32>().unwrap();
            let right = a[1].replace(")", "").parse::<u32>().unwrap();
            sum += left * right;
        } else {
            allowed = match &cap[0] {
                "don't()" => false,
                "do()" => true,
                _ => allowed
            }
        }
    }

    println!("part two: {}", sum);

    Ok(())
}

use std::{fs::read_to_string, io};
fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?
        .split("\n")
        .map(|e| e.to_string())
        .filter(|e| e != "")
        .collect::<Vec<String>>()
        .into_iter()
        .map(|e| e.split(" ").map(|e| e.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut safe_count = 0;

    for report in &input {
        if check_safe(report) {
            safe_count += 1;
        }
    }

    println!("part one: {}", safe_count);

    let mut safe2 = 0;

    for report in &input {
        let mut safe = false;
        for i in 0..report.len() {
            let mut new = report.clone();
            new.remove(i);
            if check_safe(&new) {
                safe = true;
            }
        }
        if safe {
            safe2 += 1;
        }
    }
    println!("part two: {}", safe2);

    Ok(())
}

fn check_safe(report: &Vec<u32>) -> bool {

        let mut decreasing = false;
        let mut safe = true;
        if report[0] > report[1] {
            decreasing = true;
        }

        let mut current = report[0];
        for level in &report[1..] {
            if current.abs_diff(*level) <= 3 {
                if decreasing {
                    if current > *level {
                        current = *level
                    } else {
                        safe = false;
                    }
                } else {
                    if current < *level {
                        current = *level
                    } else {
                        safe = false;
                    }
                }
            } else {
                safe = false;
                break;
            }
        }
        safe
}

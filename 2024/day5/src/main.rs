use std::{collections::HashMap, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;
    let instructions_vec = input.split("\n\n").collect::<Vec<_>>()[0].split("\n").collect::<Vec<_>>();
    let updates_vec = input.split("\n\n").collect::<Vec<_>>()[1].split("\n").filter(|e| *e != "").collect::<Vec<_>>();

    let instruction = make_instruction(instructions_vec);
    let mut sum = 0;
    let mut sum2 = 0;

    for update in &updates_vec {
        let pages = update.split(",").collect::<Vec<_>>();
        if check_if_right(&pages, &instruction, pages.len()) {
            sum += pages.get(pages.len() / 2).unwrap().parse::<u32>().unwrap();
        } else {
            let mut cloni = pages.clone();
            cloni = try_sort(cloni, &instruction, pages.len());
            sum2 += cloni.get(cloni.len() / 2).unwrap().parse::<u32>().unwrap();
        }
    }
    println!("print one: {}", sum);
    println!("print two: {}", sum2);


    Ok(())
}

fn make_instruction<'a>(instructions_vec: Vec<&'a str>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut instruction: HashMap<&str, Vec<&str>> = HashMap::new();

    for inst in instructions_vec {
        let before = inst.split("|").collect::<Vec<_>>()[0];
        let after = inst.split("|").collect::<Vec<_>>()[1];

        match instruction.get_mut(before){
            Some(value) => value.push(after),
            None => {
                instruction.insert(before, vec![after]);
            }
        }
    }

    instruction
}

fn check_if_right(pages: &Vec<&str>, instruction: &HashMap<&str, Vec<&str>>, len: usize) -> bool {
        let mut good = true;
        for i in 0..len {
            for j in i+1..len {
                let list = match instruction.get(pages[j]) {
                    Some(l) => l,
                    None => continue
                };
                if list.contains(&pages[i]) {
                    good = false;
                }
            }
        }
        good
}

fn try_sort<'a>(mut pages: Vec<&'a str>, instruction: &HashMap<&str, Vec<&str>>, len: usize) -> Vec<&'a str> {
    while !check_if_right(&pages, instruction, len) {
        for i in 0..len {
            for j in i+1..len {
                let list = match instruction.get(pages[j]) {
                    Some(l) => l,
                    None => continue
                };
                if list.contains(&pages[i]) {
                    let temp = pages[j];
                    pages[j] = pages[i];
                    pages[i] = temp;
                }
            }
        }
    }

    pages
}

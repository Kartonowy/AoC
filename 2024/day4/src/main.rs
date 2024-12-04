use std::{fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;
    let lines = input.split("\n").filter(|x| *x != "").collect::<Vec<_>>();
    let mut counter = 0;
    let mut xcounter = 0;

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i].chars().nth(j).unwrap() == 'X' {
                if check_forward(lines[i], j) {
                    counter += 1
                }
                if check_backwards(lines[i], j) {
                    counter += 1
                }
                if check_vertical_forward(&lines, i, j) {
                    counter += 1
                }
                if check_vertical_backward(&lines, i, j) {
                    counter += 1
                }
                if check_diagonal_1(&lines, i, j) {
                    counter += 1
                }
                if check_diagonal_2(&lines, i, j) {
                    counter += 1
                }
                if check_diagonal_3(&lines, i, j) {
                    counter += 1
                }
                if check_diagonal_4(&lines, i, j) {
                    counter += 1
                }
            }
            if lines[i].chars().nth(j).unwrap() == 'A' {
                if check_x(&lines, i, j) {
                    xcounter += 1
                }
            }
        }
    }

    println!("task one: {}", counter);
    println!("task two: {}", xcounter);

    Ok(())
}

fn check_forward(line: &str, j: usize) -> bool {
    return line.chars().nth(j + 1).unwrap_or('C') == 'M'
        && line.chars().nth(j + 2).unwrap_or('C') == 'A'
        && line.chars().nth(j + 3).unwrap_or('C') == 'S';
}

fn check_backwards(line: &str, j: usize) -> bool {
    if j < 3 {
        return false;
    }
    return line.chars().nth(j - 1).unwrap_or('C') == 'M'
        && line.chars().nth(j - 2).unwrap_or('C') == 'A'
        && line.chars().nth(j - 3).unwrap_or('C') == 'S';
}
fn check_vertical_forward(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i > lines.len() - 4 {
        return false;
    }
    return lines[i + 1].chars().nth(j).unwrap_or('C') == 'M'
        && lines[i + 2].chars().nth(j).unwrap_or('C') == 'A'
        && lines[i + 3].chars().nth(j).unwrap_or('C') == 'S';
}

fn check_vertical_backward(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    return lines[i - 1].chars().nth(j).unwrap_or('C') == 'M'
        && lines[i - 2].chars().nth(j).unwrap_or('C') == 'A'
        && lines[i - 3].chars().nth(j).unwrap_or('C') == 'S';
}

fn check_diagonal_1(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i > lines.len() - 4 {
        return false;
    }
    if j < 3 {
        return false;
    }
    return lines[i + 1].chars().nth(j - 1).unwrap_or('C') == 'M'
        && lines[i + 2].chars().nth(j - 2).unwrap_or('C') == 'A'
        && lines[i + 3].chars().nth(j - 3).unwrap_or('C') == 'S';
}
fn check_diagonal_2(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i < 3 || j < 3 {
        return false;
    }
    return lines[i - 1].chars().nth(j - 1).unwrap_or('C') == 'M'
        && lines[i - 2].chars().nth(j - 2).unwrap_or('C') == 'A'
        && lines[i - 3].chars().nth(j - 3).unwrap_or('C') == 'S';
}
fn check_diagonal_3(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i < 3 {
        return false;
    }
    return lines[i - 1].chars().nth(j + 1).unwrap_or('C') == 'M'
        && lines[i - 2].chars().nth(j + 2).unwrap_or('C') == 'A'
        && lines[i - 3].chars().nth(j + 3).unwrap_or('C') == 'S';
}
fn check_diagonal_4(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if i > lines.len() - 4 {
        return false;
    }
    return lines[i + 1].chars().nth(j + 1).unwrap_or('C') == 'M'
        && lines[i + 2].chars().nth(j + 2).unwrap_or('C') == 'A'
        && lines[i + 3].chars().nth(j + 3).unwrap_or('C') == 'S';
}

fn check_x(lines: &Vec<&str>, i: usize, j: usize) -> bool {
    if j < 1 || i < 1 || i > lines.len() - 2 {
        return false;
    }
    return ((lines[i - 1].chars().nth(j - 1).unwrap_or('C') == 'M'
        && lines[i + 1].chars().nth(j + 1).unwrap_or('C') == 'S')
        || (lines[i + 1].chars().nth(j + 1).unwrap_or('C') == 'M'
            && lines[i - 1].chars().nth(j - 1).unwrap_or('C') == 'S'))
        && ((lines[i + 1].chars().nth(j - 1).unwrap_or('C') == 'M'
            && lines[i - 1].chars().nth(j + 1).unwrap_or('C') == 'S')
            || (lines[i - 1].chars().nth(j + 1).unwrap_or('C') == 'M'
                && lines[i + 1].chars().nth(j - 1).unwrap_or('C') == 'S'));
}

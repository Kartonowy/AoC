use core::panic;
use std::{fs::read_to_string, io};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}
fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;

    let mut map = input
        .split('\n')
        .filter(|e| *e != "")
        .map(|e| e.chars().collect())
        .collect::<Vec<Vec<char>>>();

    task_one(map.clone());




    Ok(())

}

fn task_one(map: Vec<Vec<char>>) {
    let mut pos = find_guard(&map);
    loop {

        if check_bounds(&map, pos) {
            map[pos.y][pos.x] = '+';
            break;
        }

        match map[pos.y][pos.x] {
            '^' => {
                if map[pos.y - 1][pos.x] != '#' {
                    map[pos.y - 1][pos.x] = '^';
                    map[pos.y][pos.x] = '|';
                    pos.y -= 1;
                } else {
                    map[pos.y][pos.x] = '>';
                }
            },
            '>' => {
                if map[pos.y][pos.x + 1] != '#' {
                    map[pos.y][pos.x + 1] = '>';
                    map[pos.y][pos.x] = '-';
                    pos.x += 1;
                } else {
                    map[pos.y][pos.x] = 'v';
                }

            },
            '<' => {
                if map[pos.y][pos.x - 1] != '#' {
                    map[pos.y][pos.x - 1] = '<';
                    map[pos.y][pos.x] = '-';
                    pos.x -= 1;
                } else {
                    map[pos.y][pos.x] = '^';
                }

            },
            'v' => {
                if map[pos.y + 1][pos.x] != '#' {
                    map[pos.y + 1][pos.x] = 'v';
                    map[pos.y][pos.x] = '|';
                    pos.y += 1;
                } else {
                    map[pos.y][pos.x] = '<';
                }

            },
            '-' | '+' | '|' => {},
            e => { panic!("panic found {}", e) }
        }
        println!("part one: {}", count_Xs(&map));
        print_map(&map);
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> Point {
    let mut pos = Point { x: 0, y: 0};
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos.x = x;
                pos.y = y;
            }
        }
    }
    pos
}

fn print_map(map: &Vec<Vec<char>>) {
    for vex in map {
        for cell in vex {
            print!("{}", cell);
        }
        print!("\n");
    }
    print!("\n");
}

fn check_bounds(map: &Vec<Vec<char>>, pos: Point) -> bool {
        return match map[pos.y][pos.x] {
            '^' => { pos.y  <= 0 },
            '>' => { pos.x  > map[0].len() - 2 },
            '<' => { pos.x  <= 0 },
            'v' => { pos.y  > map.len() - 2 },
            c => { panic!("panic guard found {}", c) },
        }
}

fn count_Xs(map: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    for vex in map {
        for cell in vex {
            if *cell == '|' || *cell == '-' || *cell == '+' {
                sum += 1;
            }
        }
    }
    sum
}

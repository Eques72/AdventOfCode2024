use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_file(rows: &mut Vec<Vec<char>>) -> io::Result<()> 
{
    let file_path = "input_task_6.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() 
    {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        rows.push(row);
    }
    Ok(())
}

fn main() -> io::Result<()>
{
    let part = 1;
    let mut rows: Vec<Vec<char>> = Vec::new();

    read_file(&mut rows)?;

    // for row in rows.clone() {
    //     println!("{:?}", row);
    // }
    let height = rows.len();
    let width = rows[0].len();

    let mut direction = Direction::Up;
    let mut position = (0, 0);
    //find guard
    for i in 0..height {
        for j in 0..width {
            //Assuming that the guard is always starting looking up
            if rows[i][j] == '^' {
                position = (i, j); //Y, X or Height, Width or Row, Column
                break;
            }
        }
    }
    if part == 1{
        let mut total_steps : i32 = 0;
        let mut visited_places: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_on_duty = true;
        while guard_on_duty
        {
            // println!("Position: {:?}, Direction: {:?}", position, direction);
            visited_places.insert((position.0, position.1));
            match direction {
                Direction::Up => 
                {
                    if position.0 == 0 {
                        guard_on_duty = false;
                    } else {
                        if rows[position.0 - 1][position.1] == '#' {
                            direction = Direction::Right;
                        }
                        else {
                            position.0 -= 1;
                            total_steps += 1;
                        }
                    }
                }
                Direction::Down =>                 
                {
                    if position.0 == height - 1 {
                        guard_on_duty = false;
                    } else {
                        if rows[position.0 + 1][position.1] == '#' {
                            direction = Direction::Left;
                        }
                        else {
                            position.0 += 1;
                            total_steps += 1;
                        }
                    }
                }
                Direction::Left =>
                {
                    if position.1 == 0 {
                        guard_on_duty = false;
                    } else {
                        if rows[position.0][position.1 - 1] == '#' {
                            direction = Direction::Up;
                        }
                        else {
                            position.1 -= 1;
                            total_steps += 1;
                        }
                    }
                }
                Direction::Right =>
                {
                    if position.1 == width - 1 {
                        guard_on_duty = false;
                    } else {
                        if rows[position.0][position.1 + 1] == '#' {
                            direction = Direction::Down;
                        }
                        else {
                            position.1 += 1;
                            total_steps += 1;
                        }
                    }
                }
            }
        }
        println!("Total steps: {:?}", total_steps);
        println!("Visited places: {:?}", visited_places.len());
    }
    if part == 2{
    }
    Ok(())
}

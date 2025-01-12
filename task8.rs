use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn read_file(rows: &mut Vec<Vec<char>>) -> io::Result<()> 
{
    let file_path = "input_task_8.txt";
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
    let part = 2;
    let mut rows: Vec<Vec<char>> = Vec::new();

    read_file(&mut rows)?;

    let mut sources: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut signals: HashSet<(i32, i32)> = HashSet::new();

    let height = rows.len();
    let width = rows[0].len();
    for i in 0..height {
        for j in 0..width {
            if rows[i][j].is_ascii_digit() || rows[i][j].is_ascii_uppercase() || rows[i][j].is_ascii_lowercase()
            {
                if sources.contains_key(&rows[i][j])
                {
                    sources.get_mut(&rows[i][j]).expect("REASON").push((i as i32, j as i32));
                }
                else
                {
                    sources.insert(rows[i][j], vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    if part == 1{
        for (key, value) in &sources {
            for (i, j) in value {
                for (k, m) in value {
                    if i == k && j == m
                    {
                        continue;
                    }
                    let delta_x = i - k;
                    let delta_y = j - m;
                    let new_i = i + delta_x;
                    let new_j = j + delta_y;
                    if new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32
                    {
                        signals.insert((new_i as i32, new_j as i32));
                    }
                }
            }
        }
    }
    else
    {
        for (key, value) in &sources {
            for (i, j) in value {
                for (k, m) in value {
                    if i == k && j == m
                    {
                        continue;
                    }
                    signals.insert((*i as i32, *j as i32));
                    let delta_x = i - k;
                    let delta_y = j - m;
                    let mut new_i = i + delta_x;
                    let mut new_j = j + delta_y;
                    while new_i >= 0 && new_i < height as i32 && new_j >= 0 && new_j < width as i32
                    {
                        signals.insert((new_i as i32, new_j as i32));
                        // rows[new_i as usize][new_j as usize] = '#';
                        new_i = new_i + delta_x;
                        new_j = new_j + delta_y;
                    }
                }
            }
        }    
    }


    println!("Signals: {}", signals.len());

    // println!("{:?}", rows);

    Ok(())
}

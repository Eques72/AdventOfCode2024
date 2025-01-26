use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn read_file() -> io::Result<Vec<Vec<i32>>> {
    let file_path = "input_task_10.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut rows_vec: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() 
    {
        let line = line?;
        let row_int: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
        rows_vec.push(row_int);
    }
    Ok(rows_vec)
}

fn search_path(trail: Vec<Vec<i32>>, peaks: &mut HashSet<(usize, usize)>, y: usize, x: usize, step: i32, task_one: bool) -> i32
{
    if step == 9
    {   
        if task_one
        {
            peaks.insert((y, x));
            return peaks.len() as i32;    
        }
        return 1;
    }
    let mut paths:i32 = 0;
    //four directions
    if y > 0 && trail[y-1][x] == step + 1
    {
        paths += search_path(trail.clone(), peaks, y-1, x, step + 1, task_one);
    }
    if y < trail.len() - 1 && trail[y+1][x] == step + 1
    {
        paths += search_path(trail.clone(), peaks, y+1, x, step + 1, task_one);
    }
    if x > 0 && trail[y][x-1] == step + 1
    {
        paths += search_path(trail.clone(), peaks, y, x-1, step + 1, task_one);
    }
    if x < trail[y].len() - 1 && trail[y][x+1] == step + 1
    {
        paths += search_path(trail.clone(), peaks, y, x+1, step + 1, task_one);
    }
    return if task_one { peaks.len() as i32 } else { paths };
}

fn main() -> io::Result<()>
{
    let is_part_one = false;
    let trails = read_file().unwrap();

    let mut total_paths:i32 = 0;
    for i in 0..trails.len()
    {
        for j in 0..trails[i].len()
        {
            if trails[i][j] == 0
            {
                let mut peaks: HashSet<(usize, usize)> = HashSet::new();
                let paths: i32 = search_path(trails.clone(), &mut peaks, i, j, 0, is_part_one);
                println!("{:?}", paths);
                total_paths += paths;
            }
        }
    }
    println!("{:?}", total_paths);

    Ok(())
}

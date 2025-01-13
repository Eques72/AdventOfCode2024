use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file() -> io::Result<Vec<i32>> {
    let file_path = "input_task_9.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let vector: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    Ok(vector)
}

fn find_last_data(vec: &Vec<i32>, start_from: usize) -> usize
{
    let mut i: usize = start_from;
    while vec[i] == -1
    {
        i -= 1;
    }
    return i;
}

fn find_first_space(vec: &Vec<i32>, start_from: usize) -> usize
{
    let mut i: usize = start_from;
    while vec[i] != -1
    {
        i += 1;
    }
    return i;
}

fn main() -> io::Result<()>
{
    let part = 1;
    let disc_coded = read_file().unwrap();
    let mut disc: Vec<i32> = Vec::with_capacity(disc_coded.len()*5);

    let mut current_id: i32 = 0;
    for i in 0..disc_coded.len()
    {
        if i % 2 == 0
        {
            disc.extend(std::iter::repeat(current_id).take(disc_coded[i] as usize));
            current_id += 1;
        }
        else
        {
            disc.extend(std::iter::repeat(-1).take(disc_coded[i] as usize));
        }
    }

    let mut free_space_ind = find_first_space(&disc, 0);
    let mut data_ind = find_last_data(&disc, disc.len()-1);

    while free_space_ind < data_ind
    {
        disc[free_space_ind] = disc[data_ind];
        disc[data_ind] = -1;

        free_space_ind = find_first_space(&disc, 0);
        data_ind = find_last_data(&disc, disc.len()-1);    
    }

    let mut sum: i64 = 0;
    for i in 0..disc.len()
    {
        if disc[i] == -1
        { break; }
        // println!("sum {:?} {:?}", i, disc[i]);
        sum += (i as i64) * (disc[i] as i64);
    }
    println!("Control sum {:?}", sum);
    // println!("{:?}", disc);

    Ok(())
}

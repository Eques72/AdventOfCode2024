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
        if i == 0
        {break;}
    }
    return i;
}

fn find_first_space(vec: &Vec<i32>, start_from: usize) -> usize
{
    let mut i: usize = start_from;
    while vec[i] != -1
    {
        i += 1;
        if i == vec.len()
        {break;}
    }
    return i;
}

fn main() -> io::Result<()>
{
    let part = 2;
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

    if part == 1
    {
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
    }
    else
    {
        let mut space_list: Vec<(usize, i32)> = Vec::new();
        
        let mut free_space_ind = find_first_space(&disc, 0);
        while free_space_ind < disc.len()
        {
            let mut free_space_end = free_space_ind;
            while disc[free_space_end] == -1
            {
                free_space_end += 1;
                if free_space_end == disc.len()
                {break;}
            }
            space_list.push((free_space_ind, (free_space_end - free_space_ind) as i32));
            free_space_ind = find_first_space(&disc, free_space_end);
        }

        let mut data_ind = find_last_data(&disc, disc.len()-1);
        while data_ind != 0
        {
            let mut data_end_ind = data_ind;
            while disc[data_end_ind] == disc[data_ind]
            {
                data_end_ind -= 1;
                if data_end_ind == 0
                {break;}
            }
            let file_size = data_ind - data_end_ind;

            for i in 0..space_list.len()
            {
                if space_list[i].1 >= file_size as i32 && space_list[i].0 <= data_end_ind
                {
                    // println!("{:?}", disc);
                    for i in space_list[i].0..(space_list[i].0+file_size)
                    { disc[i] = disc[data_ind]; }
                    for i in (data_end_ind+1)..(data_ind+1)
                    { disc[i] = -1; }
                    space_list[i] = (space_list[i].0+file_size, space_list[i].1-(file_size as i32));
                    // println!("{:?}", disc);
                    // println!("{:?}", space_list);
                    break;
                }
            }
            data_ind = find_last_data(&disc, data_end_ind);
        }


        let mut sum: i64 = 0;
        for i in 0..disc.len()
        {
            if disc[i] == -1
            { continue; }
            sum += (i as i64) * (disc[i] as i64);
        }
        println!("Control sum {:?}", sum);
        // println!("{:?}", disc);
    }



    Ok(())
}

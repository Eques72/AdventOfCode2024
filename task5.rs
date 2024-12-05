use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file(pairs: &mut Vec<(i32, i32)>, lists: &mut Vec<Vec<i32>>) -> io::Result<()> 
{
    let file_path = "input_task_5.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut read_rules = true;
    for line in reader.lines() 
    {
        let line = line?;
        if line.trim().is_empty() 
        {
            read_rules = false;
            continue;
        }

        if read_rules 
        {
            if let Some((first, second)) = line.split_once('|') 
            {
                let first: i32 = first.trim().parse().expect("Error: Not a int");
                let second: i32 = second.trim().parse().expect("Error: Not a int");
                pairs.push((first, second));
            }
        } else {
            let list: Vec<i32> = line
                .split(',')
                .map(|num| num.trim().parse().expect("Error: Not a int"))
                .collect();
            lists.push(list);
        }
    }
    Ok(())
}

fn main() -> io::Result<()>
{
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    let mut lists: Vec<Vec<i32>> = Vec::new();

    read_file(&mut pairs, &mut lists)?;

    let mut correct_pages: Vec<Vec<i32>> = Vec::new();
    'pages: for list in &lists {
        for i in 0..list.len() {
            //each number must be checked to fulfill every rule that concerns it (is a first number in the pair)
            for (l, r) in &pairs 
            {
                if *l == list[i]
                {
                    for j in 0..i 
                    {
                        if list[j] == *r
                        {
                            println!("Not correct! {:?} by rule {:?} | {:?}", list, *l, *r);
                            continue 'pages;
                        }
                    }
                }
            }
        }
        correct_pages.push(list.clone());
    }
    println!("lists len: {:?}", lists.len());
    println!("correct_pages len: {:?}", correct_pages.len());

    let mut sum_of_middle_vals = 0;
    for list in &correct_pages 
    {
        println!("index: {:?}, len {:?}, value {:?}", (list.len()-1)/2,list.len(), list[(list.len()-1)/2]);
        sum_of_middle_vals += list[(list.len()-1)/2];
    }
    println!("sum_of_middle_vals: {:?}", sum_of_middle_vals);
    Ok(())
}

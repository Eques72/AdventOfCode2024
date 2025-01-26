use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file() -> io::Result<Vec<Vec<char>>> {
    let file_path = "input_task_11.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let vector: Vec<Vec<char>> = line
        .trim()
        .split_whitespace()
        .map(|stone| stone.chars().collect())
        .collect();
    Ok(vector)
}

fn vec_of_chars_to_int(chars: Vec<char>) -> i64 
{
    let num_str: String = chars.into_iter().collect();    
    return num_str.parse::<i64>().expect("REASON");
}

fn int_to_vec_of_chars(integer: i64) -> Vec<char> 
{
    return integer.to_string().chars().collect();
}

// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

fn apply_rule_1(stone: &mut Vec<char>) -> bool
{
    if stone.len() == 1 && stone[0] == '0'
    {
        stone[0] = '1';
        return true;
    }
    return false;
}

fn apply_rule_2(stone: &mut Vec<char>) -> Vec<char>
{
    if stone.len() % 2 == 0
    {
        let mut new_stone = stone.split_off(stone.len() / 2);
        while new_stone.len() > 1 && new_stone[0] == '0'
        {
            new_stone.remove(0);
        }
        return new_stone
    }
    return Vec::<char>::new();
}

fn apply_rule_3(stone: &mut Vec<char>)
{
    let mut int: i64 = vec_of_chars_to_int(stone.clone());
    int *= 2024;
    stone.clear();
    stone.extend(int_to_vec_of_chars(int));
}

fn main() -> io::Result<()>
{
    let is_part_one = true;
    let mut stones = read_file().unwrap();

    println!("{:?}", stones[0]);
    println!("{:?}", stones[0][0]);

    let number_of_blinks = 25;

    for _i in 0..number_of_blinks
    {
        let mut stones_to_insert = Vec::new();
        let mut line_len = stones.len();
        for j in 0..line_len
        {
            if !apply_rule_1(&mut stones[j])
            {
                let new_stone = apply_rule_2(&mut stones[j]);
                if new_stone.len() != 0
                {
                    stones_to_insert.push((new_stone, j+1))
                }
                else
                {
                    apply_rule_3(&mut stones[j]);
                }
            }
        }
        let mut stones_inserted: usize = 0;
        for new_stone in stones_to_insert
        {
            stones.insert(new_stone.1+stones_inserted, new_stone.0);
            stones_inserted += 1;
        }

        line_len = stones.len();
        println!("LEN: {:?}", line_len);

        // for s in stones.clone()
        // {
        //     for (i, &ch) in s.iter().enumerate() {
        //         print!("{}", ch);
        //     }
        //     print!(" ");
        // }
        // println!();
    }

    Ok(())
}

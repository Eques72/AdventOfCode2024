use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn read_file() -> io::Result<Vec<i128>> {
    let file_path = "input_task_11.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let mut vec = Vec::new();

    vec.extend(line.split_whitespace()
        .filter_map(|s| s.parse::<i128>().ok()));
    
    Ok(vec)
}

fn count_digits(integer: i128) -> usize {
    let mut count = 0;
    let mut num = integer;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    return count;
}

// // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

fn split_number_on_half_cache(integer: i128, threshold: usize) -> (i128, i128) 
{
    let divisor = 10_i128.pow((threshold) as u32);
    return (integer / divisor, integer % divisor);
 }

fn apply_rule_1_cache(stone: i128, cache: &mut HashMap<i128, i128>, old_cache: &HashMap<i128, i128>) -> bool
{
    if stone == 0
    {
        *cache.entry(1).or_insert(0) += old_cache[&0];
        return true;
    }
    return false;
}

fn apply_rule_2_cache(stone: i128, cache: &mut HashMap<i128, i128>, old_cache: &HashMap<i128, i128>) -> bool
{
    let threshold = count_digits(stone);
    if threshold % 2 == 0
    {
        let stones_lr = split_number_on_half_cache(stone, threshold/2);
        *cache.entry(stones_lr.0).or_insert(0) += old_cache[&stone];
        *cache.entry(stones_lr.1).or_insert(0) += old_cache[&stone];
        return true;
    }
    return false;
}

fn apply_rule_3_cache(stone: i128, cache: &mut HashMap<i128, i128>, old_cache: &HashMap<i128, i128>)
{
    *cache.entry(stone * 2024).or_insert(0) += old_cache[&stone];
}

fn print_stones_map(stones_map: HashMap<i128,i128>)
{
    for (s, v) in stones_map
    {
        println!("Stone of number {:?} exists {:?} times", s, v);
    }
    println!();
}

fn main() -> io::Result<()>
{
    let stones = read_file().unwrap();

    let mut cache_multi = HashMap::<i128,i128>::new();

    println!("{:?}", stones);

    let number_of_blinks = 75;

    for stone in stones
    {
        cache_multi.insert(stone, 1);
    }

    print_stones_map(cache_multi.clone());

    for _i in 0..number_of_blinks
    {
        let mut new_cache = HashMap::<i128,i128>::new();
        for (stone, _n_o_s) in &cache_multi
        {
            if !apply_rule_1_cache(*stone, &mut new_cache, &cache_multi)
            {
                if !apply_rule_2_cache(*stone, &mut new_cache, &cache_multi)
                {
                    apply_rule_3_cache(*stone, &mut new_cache, &cache_multi);
                }
            }
        }
        cache_multi = new_cache;
    }
    let mut total: i128 = 0;
    for (_stone, n_o_s) in &cache_multi
    {
        total += n_o_s;
    }
    println!("TOTAL {:?}", total);

    Ok(())
}

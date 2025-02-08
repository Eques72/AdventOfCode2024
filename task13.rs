use std::fs::File;
use std::io::{self, BufRead};

fn read_file(tuples: &mut Vec<(((i64, i64), (i64, i64)), (i64, i64))>, apply_modifier: bool)-> io::Result<()>  
{
    let file_path = "input_task_13.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    
    let mut buffer = Vec::new();

    for line in reader.lines() 
    {
        let line = line?;
        if line.trim().is_empty() 
        {
            if !buffer.is_empty() 
            {
                tuples.push(parse_tuple(&buffer, apply_modifier)?);
                buffer.clear();
            }
        } 
        else 
        {
            buffer.push(line);
        }
    }

    if !buffer.is_empty() {
        tuples.push(parse_tuple(&buffer, apply_modifier)?);
    }

    Ok(())
}


fn parse_tuple(lines: &[String], apply_modifier: bool) -> io::Result<(((i64, i64), (i64, i64)), (i64, i64))> {
    let mut a_coords = (0, 0);
    let mut b_coords = (0, 0);
    let mut x_y = (0, 0);

    for line in lines {
        let simplified = line.replace("Button A: X+", "A ")
                            .replace(", Y+", " ")
                            .replace("Button B: X+", "B ")
                            .replace("Prize: X=", "P ")
                            .replace(", Y=", " ");

        let parts: Vec<&str> = simplified.split_whitespace().collect();        
        if parts.len() == 3 {
            let x = parts[1].parse::<i64>().unwrap_or(0);
            let y = parts[2].parse::<i64>().unwrap_or(0);
            match parts[0] {
                "A" => a_coords = (x, y),
                "B" => b_coords = (x, y),
                "P" => x_y = (x, y),
                _ => {}
            }
        }
    }
    if apply_modifier
    {
        x_y.0 += 10000000000000; x_y.1 += 10000000000000; 
    }

    Ok(((a_coords, b_coords), x_y))
}

fn find_single_axis_arcade_solutions_1(A: i64, B: i64, C: i64, max_100: bool) -> Vec<(i64,i64)>
{
    let mut solutions: Vec<(i64,i64)> = Vec::new();
    for x in 0..((C/A)+1)
    {  
        if max_100 && x > 100
        {
            break;
        }
        let remainder = C - x * A;

        if remainder % B == 0
        {
            let y = remainder / B;
            if y >= 0 && (!max_100 || y <= 100){ solutions.push((x, y)) }
        }
    }
    return solutions
}

fn find_arcade_solutions_2(target: (i64,i64), A: (i64,i64), B: (i64,i64)) -> Option<(i64,i64)>
{
    let D = A.0 * B.1 - A.1 * B.0;
    if D == 0 { return None }

    let D_A =  target.0 * B.1 - target.1 * B.0;
    let D_B =  A.0 * target.1 - A.1 * target.0;

    if D_A % D != 0 || D_B % D != 0 { return None }

    let C_A = D_A / D;
    let C_B = D_B / D;
    if C_A < 0 || C_B < 0 { return None }
    return Some((C_A, C_B))
}

fn find_cost_effective_solution(solutions: Vec<(i64,i64)>) -> i64
{
    let mut min_s = solutions[0].0 * 3 + solutions[0].1;
    for s in solutions
    {
        let cost = s.0 * 3 + s.1;
        if cost < min_s {min_s = cost}
    }
    min_s
}

fn match_pairs(solutions_x: Vec<(i64,i64)>, solutions_y: Vec<(i64,i64)>) -> Vec<(i64,i64)>
{
    let mut matching: Vec<(i64,i64)> = Vec::new();
    for x in solutions_x
    {
        for y in &solutions_y
        {
            if x.0 == y.0 && x.1 == y.1
            {
                matching.push(x);
            }
        }
    }
    matching
}

fn main() -> io::Result<()>
{
    let is_task_one: bool = false;
    let mut machines: Vec<(((i64, i64), (i64, i64)), (i64, i64))> = Vec::new();
    read_file(&mut machines, !is_task_one)?;

    let mut total_costs = 0;
    for m in machines
    {
        if is_task_one
        {
            let found = match_pairs(find_single_axis_arcade_solutions_1(m.0.0.0,m.0.1.0,m.1.0, is_task_one), find_single_axis_arcade_solutions_1(m.0.0.1,m.0.1.1,m.1.1, is_task_one));
            if found.len() == 0 { continue; }
            let cost = find_cost_effective_solution(found.clone());

            total_costs += cost;
        }
        else
        {
            let solution = find_arcade_solutions_2(m.1, m.0.0, m.0.1).unwrap_or((0,0));
            let cost = solution.0 * 3 + solution.1;
            total_costs += cost;
        }
    }
    println!("TOTAL: {:?}", total_costs);
    Ok(())
}

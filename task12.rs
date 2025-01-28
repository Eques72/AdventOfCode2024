use std::fs::File;
use std::io::{self, BufRead};
use image::{ImageBuffer, Rgb};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Face
{
    W,
    E,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     
    S,
    N,
    NONE,
}

fn read_file(rows: &mut Vec<Vec<char>>) -> io::Result<()> 
{
    let file_path = "input_task_12.txt";
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

fn draw_image(data: &Vec<Vec<char>>) 
{
    let mut color_map: HashMap<char, (u8, u8, u8)> = HashMap::new();

    for (i, letter) in ('A'..='Z').enumerate() {
        let r = (i * 7) as u8 % 255;
        let g = (i * 17) as u8 % 255; 
        let b = (i * 29) as u8 % 255;

        color_map.insert(letter, (r, g, b));
    }

    let plot_size = 10;

    let img_width = data[0].len() * plot_size;
    let img_height = data.len() * plot_size;
    let mut img = ImageBuffer::new(img_width as u32, img_height as u32);

    for (row, row_data) in data.iter().enumerate() 
    {
        for (col, &character) in row_data.iter().enumerate() 
        {
            for x in 0..plot_size {
                for y in 0..plot_size {
                    let px = (col * plot_size + x) as u32;
                    let py = (row * plot_size + y) as u32;
                    img.put_pixel(px, py, Rgb([color_map[&character].0, color_map[&character].1, color_map[&character].2]));
                }
            }
        }
    }

    img.save("fields_visualized.png").unwrap();
}

fn measure_plots(field: &Vec<Vec<char>>, row: usize, col: usize, checked_plots: &mut HashSet<(usize, usize)>) -> (i32, i32)
{
    if checked_plots.contains(&(row, col))
    {
        return (0,0)
    }
    checked_plots.insert((row, col));

    let char_field: char = field[row][col];
    let mut area: i32 = 1;
    let mut perimeter: i32 = 0;

    if row > 0 && field[row-1][col] == char_field
    {
        let res: (i32,i32) = measure_plots(field, row-1, col, checked_plots);
        area += res.0;
        perimeter += res.1;
    }
    else
    {
        perimeter += 1;    
    }

    if row < field.len() - 1 && field[row+1][col] == char_field
    {
        let res: (i32,i32) = measure_plots(field, row+1, col, checked_plots);
        area += res.0;
        perimeter += res.1;
    }
    else
    {
        perimeter += 1;    
    }

    if col > 0 && field[row][col-1] == char_field
    {
        let res: (i32,i32) = measure_plots(field, row, col-1, checked_plots);
        area += res.0;
        perimeter += res.1;
    }
    else
    {
        perimeter += 1;    
    }

    if col < field[row].len() - 1 && field[row][col+1] == char_field
    {
        let res: (i32,i32) = measure_plots(field, row, col+1, checked_plots);
        area += res.0;
        perimeter += res.1;
    }
    else
    {
        perimeter += 1;    
    }

    (area, perimeter)
}

fn create_line(field: &Vec<Vec<char>>, row: usize, col: usize, face: Face) -> ((usize,usize),(usize,usize), Face)
{
    //line has start, and end
    if face == Face::W || face == Face::E
    {
        let mut length_to_right = 0;
        let mut length_to_left = 0;
        while row+length_to_right < field.len() - 1 && field[row+length_to_right][col] == field[row][col]
        {
            if face == Face::W && col > 0 && field[row+length_to_right][col-1] == field[row][col]
            {
                break;
            }
            if face == Face::E && col < field[0].len() - 1 && field[row+length_to_right][col+1] == field[row][col]
            {
                break;
            }
            length_to_right += 1;
        }
        while row-length_to_left > 0 && field[row-length_to_left][col] == field[row][col]
        {
            if face == Face::W && col > 0 && field[row-length_to_left][col-1] == field[row][col]
            {
                break;
            }
            if face == Face::E && col < field[0].len() - 1 && field[row-length_to_left][col+1] == field[row][col]
            {
                break;
            }
            length_to_left += 1;
        }
        ((row-length_to_left,col),(row+length_to_right, col), face)
    } 
    else
    {
        let mut length_to_down = 0;
        let mut length_to_up = 0;
        while col+length_to_down < field[0].len() - 1 && field[row][col+length_to_down] == field[row][col]
        {
            if face == Face::N && row > 0 && field[row-1][col+length_to_down] == field[row][col]
            {
                break;
            }
            if face == Face::S && row < field.len() - 1 && field[row+1][col+length_to_down] == field[row][col]
            {
                break;
            }
            length_to_down += 1;
        }
        while col-length_to_up > 0 && field[row][col-length_to_up] == field[row][col]
        {
            if face == Face::N && row > 0 && field[row-1][col-length_to_up] == field[row][col]
            {
                break;
            }
            if face == Face::S && row < field.len() - 1 && field[row+1][col-length_to_up] == field[row][col]
            {
                break;
            }
            length_to_up += 1;
        }
        ((row,col-length_to_up),(row, col+length_to_down), face)
    }
}

fn check_surroundings(field: &Vec<Vec<char>>, row: usize, col: usize) -> (Face,Face,Face,Face)
{
    //N S W E
    let mut surroundings = (Face::NONE, Face::NONE, Face::NONE, Face::NONE);

    if row == 0 || (row > 0 && field[row-1][col] != field[row][col])
    {
        surroundings.0 = Face::N;
    }
    if row == field.len() -1 || (row < field.len() - 1 && field[row+1][col] != field[row][col])
    {
        surroundings.1 = Face::S;
    }
    if col == 0 || (col > 0 && field[row][col-1] != field[row][col])
    {
        surroundings.2 = Face::W;
    }
    if col == field[0].len() -1 || (col < field[row].len() - 1 && field[row][col+1] != field[row][col])
    {
        surroundings.3 = Face::E;
    }
    return surroundings; 
}

fn measure_plots_2(field: &Vec<Vec<char>>, 
    row: usize, col: usize, 
    checked_plots: &mut HashSet<(usize, usize)>,
    predecessor_fences: (Face,Face,Face, Face),
    lines: &mut HashSet<((usize, usize),(usize, usize), Face)>
    ) -> (i32, i32)
{
    if checked_plots.contains(&(row, col))
    {
        return (0,0)
    }
    checked_plots.insert((row, col));

    let mut area: i32 = 1;
    let mut perimeter: i32 = 0;

    let surroundings = check_surroundings(field, row, col);
    
    if surroundings.0 == Face::N && predecessor_fences.0 == Face::NONE
    {
        if !(surroundings.2 == Face::NONE && checked_plots.contains(&(row, col-1)) && check_surroundings(field, row, col-1).0 == Face::N)
        && !(surroundings.3 == Face::NONE && checked_plots.contains(&(row, col+1)) && check_surroundings(field, row, col+1).0 == Face::N)         
            {
                perimeter += 1;
                lines.insert(create_line(field, row, col, Face::N));
            }
    }
    if surroundings.1 == Face::S && predecessor_fences.1 == Face::NONE
    {
        if !(surroundings.2 == Face::NONE && checked_plots.contains(&(row, col-1)) && check_surroundings(field, row, col-1).1 == Face::S)
        && !(surroundings.3 == Face::NONE && checked_plots.contains(&(row, col+1)) && check_surroundings(field, row, col+1).1 == Face::S)         
        {
            perimeter += 1;
            lines.insert(create_line(field, row, col, Face::S));
        }
    }
    if surroundings.2 == Face::W && predecessor_fences.2 == Face::NONE
    {
        if !(surroundings.0 == Face::NONE && checked_plots.contains(&(row-1, col)) && check_surroundings(field, row-1, col).2 == Face::W)
        && !(surroundings.1 == Face::NONE && checked_plots.contains(&(row+1, col)) && check_surroundings(field, row+1, col).2 == Face::W)         
        {
            perimeter += 1;
            lines.insert(create_line(field, row, col, Face::W));
        }
    }
    if surroundings.3 == Face::E && predecessor_fences.3 == Face::NONE
    {
        if !(surroundings.0 == Face::NONE && checked_plots.contains(&(row-1, col)) && check_surroundings(field, row-1, col).3 == Face::E)
        && !(surroundings.1 == Face::NONE && checked_plots.contains(&(row+1, col)) && check_surroundings(field, row+1, col).3 == Face::E)         
        {
            perimeter += 1;
            lines.insert(create_line(field, row, col, Face::E));
        }
    }

    if surroundings.0 == Face::NONE
    {
        //can go up, passing W E
        let res =measure_plots_2(field, row-1, col, checked_plots, surroundings, lines);
        area += res.0;
        perimeter += res.1;
    }
    if surroundings.1 == Face::NONE
    {
        //can go down, passing W E
        let res = measure_plots_2(field, row+1, col, checked_plots, surroundings, lines);
        area += res.0;
        perimeter += res.1;
    } 
    if surroundings.2 == Face::NONE
    {
        //can go left, passing N S
        let res = measure_plots_2(field, row, col-1, checked_plots, surroundings, lines);
        area += res.0;
        perimeter += res.1;
    }
    if surroundings.3 == Face::NONE
    {
        //can go right, passing N S
        let res= measure_plots_2(field, row, col+1, checked_plots, surroundings, lines);
        area += res.0;
        perimeter += res.1;
    } 

    (area, perimeter)
}

fn main() -> io::Result<()>
{
    let task = 2;
    let mut field: Vec<Vec<char>> = Vec::new();
    read_file(&mut field)?;

    if task == 1
    {
        draw_image(&field);
    
        let mut checked_plots: HashSet<(usize, usize)> = HashSet::new();
        let mut costs: i32 = 0;
        for (row, row_data) in field.iter().enumerate() 
        {
            for (col, _plot) in row_data.iter().enumerate() 
            {
                let res: (i32,i32) = measure_plots(&field, row, col, &mut checked_plots);
                costs += res.0 * res.1;        
            }
        }
        println!("Total costs: {:?}", costs);
    }
    else
    {
        let mut checked_plots: HashSet<(usize, usize)> = HashSet::new();
        let mut costs_l: i32 = 0;
        for (row, row_data) in field.iter().enumerate() 
        {
            for (col, _plot) in row_data.iter().enumerate() 
            {
                let mut lines: HashSet<((usize, usize),(usize, usize), Face)> = HashSet::new();
                let res: (i32,i32) = measure_plots_2(&field, row, col, &mut checked_plots, (Face::NONE,Face::NONE,Face::NONE,Face::NONE,), &mut lines);
                costs_l += res.0 * lines.len() as i32;
                if res.0 != 0
                {
                    println!("Cost of one field of type {:?} :{:?}, area {:?}, sides {:?}", field[row][col], res.0 * lines.len() as i32 , res.0, lines.len());
                }
            }
        }
        println!("Total costs: {:?}", costs_l);
    }

    Ok(())
}

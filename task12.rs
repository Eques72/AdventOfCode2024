use std::fs::File;
use std::io::{self, BufRead};
use image::{ImageBuffer, Rgb};
use std::collections::HashMap;
use std::collections::HashSet;

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
    //find regions
    //if plot is new, add 1 to area, check surroundings 4 way, if fence condition add +1 to perimeter, if same type fire nested method 
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

fn main() -> io::Result<()>
{
    let mut field: Vec<Vec<char>> = Vec::new();
    read_file(&mut field)?;

    draw_image(&field);

    let mut checked_plots: HashSet<(usize, usize)> = HashSet::new();
    let mut costs: i32 = 0;
    for (row, row_data) in field.iter().enumerate() 
    {
        for (col, _plot) in row_data.iter().enumerate() 
        {
            let res: (i32,i32) = measure_plots(&field, row, col, &mut checked_plots);
            costs += res.0 * res.1;
            if res.0 != 0
            {
                println!("Cost of one field {:?}", res.0 * res.1)
            }            
        }
    }
    println!("Total costs: {:?}", costs);

    Ok(())
}

extern crate rand;

use std::env;
use rand::Rng;
use std::time::Duration;


fn main() {
    let prog_args: Vec<String> = env::args().collect();
    let  args_count =  prog_args.len();
    match args_count {
        3 => game_of_life(&prog_args),
        _ => show_help_message()
    }
}


fn game_of_life(prog_args: &Vec<String>) {
    let ncols = arg_to_isize(&prog_args[1]);
    let nrows = arg_to_isize(&prog_args[2]);

    let pause_length = Duration::from_millis(100);
    let mut grid = build_grid(&ncols, &nrows);
    let mut count = 0;
    loop {
        print_grid(&grid);
        println!("----------");
        println!("Cycle count: {}", count);

        grid = next_gen(&grid);
        std::thread::sleep(pause_length);
        count += 1;
    }
}

fn next_gen(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    /* Take a grid as parameters and output a new grid which represents the next
     * generation.
     */
    let mut new_grid: Vec<Vec<bool>> = Vec::new();

    for row in 0..grid.len() as isize {
        let mut new_row : Vec<bool> = Vec::new();
        for col in 0..grid[0].len() as isize {
            let new_cell = compute_cell_state(&grid, &col, &row);
            new_row.push(new_cell);
        }
        new_grid.push(new_row);
    }

    new_grid
}


fn compute_cell_state(grid: &Vec<Vec<bool>>, col: &isize, row: &isize) -> bool {
    /* Check if a cell will be alive or dead in the next generation.
     *
     * params
     * ------
     * grid: The current state of the game of life
     * col: The col coordinate of the cell that is checked
     * row: The row coordinate of the cell that is checked
     *
     * output
     * ------
     * New state of the cell
     */

    let nrows: isize = grid.len() as isize;
    let ncols: isize = grid[0].len() as isize;
    let is_alive: bool = grid[*row as usize][*col as usize];

    let is_in = |col, row| -> bool {
        row >= 0 && row < nrows && col >= 0  && col < ncols
    };

    let neighbours_index: [[isize; 2]; 8] = [[col - 1, row - 1],
                                             [col - 1, *row],
                                             [col - 1, row + 1],
                                             [*col, row - 1],
                                             [*col, row + 1],
                                             [col + 1, row - 1],
                                             [col + 1, *row],
                                             [col + 1, row + 1]];

    let alive_count = neighbours_index
        .into_iter()
        .filter(|coord| {
            if is_in(coord[0], coord[1]) {
                let (col, row) = (coord[0] as usize, coord[1] as usize);
                return grid[row][col];
            }
            false
        })
        .count();

    match alive_count{
        0...1 => false,
        2...3 if is_alive => true,
        3 if ! is_alive => true,
        _ => false
    }
}


fn build_grid(ncols: &isize, nrows: &isize) -> Vec<Vec<bool>> {
    /* Initialize a Game of life grid with random cells
     */
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for _ in 0..*nrows {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..*ncols {
            let cell = generate_cell();
            row.push(cell);
        }
        grid.push(row)
    }
    grid
}


fn arg_to_isize(arg: &String) -> isize {
    match arg.parse() {
        Ok(x) => x,
        Err(_) => panic!("A positive integer is required for ncols, nrows")
    }
}


fn show_help_message() {
   println!("Conway's Game of life Rust implementation.");
   println!("Usage: ./gameoflife ncols nrows");
   println!("where ncols and nrows are positives integers representing the grid dimensions.");
}


fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            print_cell(cell);
            print!(" ");
        }
        println!("");
    }
}


fn generate_cell() -> bool {
    let cell = rand::thread_rng().gen_range(0, 2);
    match cell {
        0 => false,
        _ => true
    }
}


fn print_cell(cell: &bool) {
    match cell {
        &true => print!("O"),
        &false => print!(".")
    }
}

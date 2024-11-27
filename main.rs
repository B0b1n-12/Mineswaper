use rand::Rng;
use std::io::{self, Write};

#[derive(Clone, Copy, Debug)]
enum Cell {
    Hidden,
    Revealed(bool), // True = mine, False = no mine
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    num_mines: usize,
}

impl Board {
    fn new(width: usize, height: usize, num_mines: usize) -> Self {
        let mut grid = vec![vec![Cell::Hidden; width]; height];
        let mut rng = rand::thread_rng();

        let mut placed_mines = 0;
        while placed_mines < num_mines {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if let Cell::Hidden = grid[y][x] {
                grid[y][x] = Cell::Revealed(true);
                placed_mines += 1;
            }
        }

        Board {
            grid,
            width,
            height,
            num_mines,
        }
    }

    fn display(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Cell::Hidden => print!("* "),
                    Cell::Revealed(true) => print!("M "), // M for mine
                    Cell::Revealed(false) => print!("- "), // - for empty
                }
            }
            println!();
        }
    }

    fn reveal(&mut self, x: usize, y: usize) -> bool {
        if let Cell::Revealed(true) = self.grid[y][x] {
            return false; // hit a mine
        }

        self.grid[y][x] = Cell::Revealed(false);
        true
    }
}

fn main() {
    let width = 10;
    let height = 10;
    let num_mines = 10;

    let mut board = Board::new(width, height, num_mines);

    println!("Welcome to Minesweeper!");
    loop {
        board.display();
        print!("Enter your move (x y): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if coords.len() != 2 {
            println!("Invalid input. Please enter two numbers.");
            continue;
        }

        let x = coords[0];
        let y = coords[1];

        if x >= width || y >= height {
            println!("Invalid coordinates. Try again.");
            continue;
        }

        if !board.reveal(x, y) {
            println!("Game Over! You hit a mine.");
            break;
        }
    }
}

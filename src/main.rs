use std::time::Instant;
use std::{thread, time::Duration, io::{stdout, Write}};
use rand::seq::SliceRandom;
use rand::thread_rng;

const WIDTH: usize = 61;  // Must be odd
const HEIGHT: usize = 31; // Must be odd

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Path,
    VisitedPath,
}

type Maze = Vec<Vec<Cell>>;

fn solve_maze(
    maze: &mut Maze,
    x: usize,
    y: usize,
    end_x: usize,
    end_y: usize,
    steps: &mut usize,
) -> bool {
    if x >= WIDTH || y >= HEIGHT {
        return false;
    }

    if let Cell::Path = maze[y][x] {
        // Mark as visited
        maze[y][x] = Cell::VisitedPath;
        *steps += 1;

        // Animate this step
        print!("\x1B[2J\x1B[H"); // Clear screen and reset cursor
        print_maze(maze);
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));

        // Base case: reached the goal
        if x == end_x && y == end_y {
            return true;
        }

        // Explore neighbors recursively
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if solve_maze(maze, nx as usize, ny as usize, end_x, end_y, steps) {
                    return true;
                }
            }
        }

        // Optional: animate backtracking (commented out by default)
        // maze[y][x] = Cell::Path;
        // print!("\x1B[2J\x1B[H");
        // print_maze(maze);
        // stdout().flush().unwrap();
        // thread::sleep(Duration::from_millis(10));
    }

    false
}

fn main() {
    let mut maze = vec![vec![Cell::Wall; WIDTH]; HEIGHT];
    carve_passages(1, 1, &mut maze);

    maze[1][0] = Cell::Path; // Start
    maze[HEIGHT - 2][WIDTH - 1] = Cell::Path; // End

    let mut steps = 0;
    let start_time = Instant::now();

    let found = solve_maze(
        &mut maze,
        0,
        1,
        WIDTH - 1,
        HEIGHT - 2,
        &mut steps,
    );

    let elapsed = start_time.elapsed();

    print!("\x1B[2J\x1B[H"); // Clear final screen
    print_maze(&maze);

    println!("\n\x1b[1mStats\x1b[0m:");
    println!("  Maze size: {} × {}", WIDTH, HEIGHT);
    println!("  Path found: {}", found);
    println!("  Steps taken: {}", steps);
    println!("  Time: {:.2?}", elapsed);
}

fn carve_passages(x: usize, y: usize, maze: &mut Maze) {
    maze[y][x] = Cell::Path;

    let mut dirs = [(0, -2), (2, 0), (0, 2), (-2, 0)];
    dirs.shuffle(&mut thread_rng());

    for (dx, dy) in dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if ny > 0 && ny < HEIGHT as isize && nx > 0 && nx < WIDTH as isize {
            if let Cell::Wall = maze[ny as usize][nx as usize] {
                maze[(y as isize + dy / 2) as usize][(x as isize + dx / 2) as usize] = Cell::Path;
                carve_passages(nx as usize, ny as usize, maze);
            }
        }
    }
}

fn print_maze(maze: &Maze) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 1 && x == 0 {
                print!("\x1b[32mS\x1b[0m"); // Green S
            } else if y == HEIGHT - 2 && x == WIDTH - 1 {
                print!("\x1b[31mE\x1b[0m"); // Red E
            } else {
                match maze[y][x] {
                    Cell::Wall => print!("\x1b[90m#\x1b[0m"),        // Dim gray wall
                    Cell::Path => print!(" "),                       // Empty space
                    Cell::VisitedPath => print!("\x1b[36m.\x1b[0m"), // Cyan path
                }
            }
        }
        println!();
    }
}
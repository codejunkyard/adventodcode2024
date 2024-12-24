use advent_lib::fetch_input;
use crossterm::ExecutableCommand;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType},
};

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/15/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";

    let mut lanternfish: Lanternfish = transform(&input);

    let count_part_1 = get_part_1(&mut lanternfish);
    println!("Part 1: Total count: {}", count_part_1);

    // let count_part_2 = get_part_2(&mut robots, grid.size, iteration);
    // println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(lanternfish: &mut Lanternfish) -> u32 {
    // Create a CursorGuard, which automatically enables raw mode and hides the cursor
    let _cursor_guard = CursorGuard::new(lanternfish.grid_height as u16);

    let mut last_key_press_time = Instant::now(); // To track time of last key press
    let mut key_is_pressed = false; // Flag to detect if the key is pressed
    let movement_delay = Duration::from_millis(50); // Time interval between repeated actions

    lanternfish.render_grid();

    loop {
        // Wait for a keyboard event
        if crossterm::event::poll(std::time::Duration::from_millis(10))
            .expect("Failed to poll event")
        {
            if let Event::Key(key_event) = crossterm::event::read().expect("Failed to read event") {
                match key_event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Right | KeyCode::Left | KeyCode::Up | KeyCode::Down => {
                        let now = Instant::now();

                        // Only handle the key if it's pressed and released within a short time window
                        if key_event.kind == crossterm::event::KeyEventKind::Press
                            && !key_is_pressed
                        {
                            key_is_pressed = true; // Key is pressed down

                            lanternfish.move_cursor(key_event.code);

                            last_key_press_time = now;
                        }

                        if key_event.kind == crossterm::event::KeyEventKind::Release {
                            // Mark the key as released so we can respond to it again
                            key_is_pressed = false;
                        }

                        // If the key is held down, continue triggering movement after the delay
                        if key_is_pressed
                            && now.duration_since(last_key_press_time) >= movement_delay
                        {
                            lanternfish.move_cursor(key_event.code);

                            last_key_press_time = now; // Update last press time after movement
                        }
                    }
                    _ => {}
                }
            }
        }

        // Optionally add a small delay to avoid 100% CPU usage
        std::thread::sleep(Duration::from_millis(5));
    }

    0
}

// fn get_part_2(grid: &mut Vec<Vec<char>>) -> u32 {
//     0
// }

struct CursorGuard {
    // The CursorGuard struct is a simple RAII guard that enables raw mode and hides the cursor when it is instantiated and disables raw mode and shows the cursor when it is dropped.
    viewport_height: u16,
}

impl CursorGuard {
    fn new(viewport_height: u16) -> Self {
        // Enable raw mode when the guard is instantiated
        crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

        // Hide the cursor
        let mut stdout = stdout();
        execute!(stdout, crossterm::cursor::Hide).unwrap();

        println!("Press the arrow keys to navigate the grid. Press 'q' to quit.");

        CursorGuard {
            // Return the guard instance
            viewport_height,
        }
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        // Ensure the cursor is restored when the program ends
        let mut stdout = stdout();

        execute!(stdout, cursor::MoveTo(0, self.viewport_height + 3),).unwrap();
        execute!(stdout, crossterm::cursor::Show).unwrap();

        // Disable raw mode when the guard is dropped
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}

#[derive(Debug)]
struct Pixel {
    coordinate: Coordinate,
    character: char,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Lanternfish {
    grid: Vec<Vec<char>>,
    grid_height: usize,
    grid_width: usize,
    input: Vec<char>,
    cursor_position: Coordinate,
    next_frame: Vec<Pixel>,
    info: String,
}

impl Lanternfish {
    // Constructor for Lanternfish
    fn new(grid: Vec<Vec<char>>, input: Vec<char>) -> Self {
        let cursor_position = grid
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                row.iter()
                    .position(|&c| c == '@')
                    .map(|col_index| Coordinate {
                        x: col_index,
                        y: row_index,
                    })
            })
            .expect("Grid must contain the '@' character");

        Lanternfish {
            grid_height: grid.len(),
            grid_width: grid[0].len(),
            grid,
            input,
            cursor_position,
            next_frame: Vec::new(),
            info: String::new(),
        }
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        self.next_frame.clear();
        self.info.clear();

        // Remove the current cursor from the grid
        self.next_frame.push(Pixel {
            coordinate: Coordinate {
                x: (self.cursor_position.x),
                y: (self.cursor_position.y),
            },
            character: '.',
        });

        // Move the cursor based on the direction
        match direction {
            KeyCode::Up if self.cursor_position.y > 0 => {
                let mut view: Vec<Pixel> = Vec::new();

                // Build a "view" of pixels to the left of the cursor until hitting '#' or '.'
                for (row_index, row) in self.grid[..self.cursor_position.y].iter().enumerate().rev()
                {
                    let cell = row[self.cursor_position.x];
                    view.push(Pixel {
                        coordinate: Coordinate {
                            x: (self.cursor_position.x),
                            y: (row_index),
                        },
                        character: cell,
                    });

                    if cell == '#' || cell == '.' {
                        break; // Stop when '#' or '.' is found
                    }
                }

                if let Some(last_pixel) = view.last() {
                    if last_pixel.character == '.' {
                        self.cursor_position.y -= 1;

                        if let Some((_last, rest)) = view.split_last() {
                            for pixel in rest {
                                self.next_frame.push(Pixel {
                                    coordinate: Coordinate {
                                        x: (pixel.coordinate.x),
                                        y: (pixel.coordinate.y - 1),
                                    },
                                    character: 'O',
                                });
                            }
                        }
                    }
                }
            }
            KeyCode::Down if self.cursor_position.y < self.grid_height - 1 => {
                let mut view: Vec<Pixel> = Vec::new();

                // Build a "view" of pixels below the cursor until hitting '#' or '.'
                for (row_index, row) in self.grid[(self.cursor_position.y + 1)..].iter().enumerate()
                {
                    let cell = row[self.cursor_position.x];

                    view.push(Pixel {
                        coordinate: Coordinate {
                            x: (self.cursor_position.x),
                            y: (self.cursor_position.y + row_index + 1),
                        },
                        character: cell,
                    });

                    if cell == '#' || cell == '.' {
                        break; // Stop when '#' or '.' is found
                    }
                }

                if let Some(last_pixel) = view.last() {
                    if last_pixel.character == '.' {
                        self.cursor_position.y += 1;

                        if let Some((_last, rest)) = view.split_last() {
                            for pixel in rest {
                                self.next_frame.push(Pixel {
                                    coordinate: Coordinate {
                                        x: (pixel.coordinate.x),
                                        y: (pixel.coordinate.y + 1),
                                    },
                                    character: 'O',
                                });
                            }
                        }
                    }
                }
            }
            KeyCode::Left if self.cursor_position.x > 0 => {
                let mut view: Vec<Pixel> = Vec::new();

                // Build a "view" of pixels to the left of the cursor until hitting '#' or '.'
                for (col_index, &cell) in self.grid[self.cursor_position.y]
                    [..self.cursor_position.x]
                    .iter()
                    .enumerate()
                    .rev()
                {
                    view.push(Pixel {
                        coordinate: Coordinate {
                            x: (col_index),
                            y: (self.cursor_position.y),
                        },
                        character: cell,
                    });

                    if cell == '#' || cell == '.' {
                        break; // Stop when '#' or '.' is found
                    }
                }

                if let Some(last_pixel) = view.last() {
                    if last_pixel.character == '.' {
                        self.cursor_position.x -= 1;

                        if let Some((_last, rest)) = view.split_last() {
                            for pixel in rest {
                                self.next_frame.push(Pixel {
                                    coordinate: Coordinate {
                                        x: (pixel.coordinate.x - 1),
                                        y: (pixel.coordinate.y),
                                    },
                                    character: 'O',
                                });
                            }
                        }
                    }
                }
            }
            KeyCode::Right if self.cursor_position.x < self.grid_width - 1 => {
                let mut view: Vec<Pixel> = Vec::new();

                // Build a "view" of pixels to the right of the cursor until hitting '#' or '.'
                for (col_index, &cell) in self.grid[self.cursor_position.y]
                    [(self.cursor_position.x + 1)..]
                    .iter()
                    .enumerate()
                {
                    view.push(Pixel {
                        coordinate: Coordinate {
                            x: (self.cursor_position.x + col_index + 1),
                            y: (self.cursor_position.y),
                        },
                        character: cell,
                    });

                    if cell == '#' || cell == '.' {
                        break; // Stop when '#' or '.' is found
                    }
                }

                if let Some(last_pixel) = view.last() {
                    if last_pixel.character == '.' {
                        self.cursor_position.x += 1;

                        if let Some((_last, rest)) = view.split_last() {
                            for pixel in rest {
                                self.next_frame.push(Pixel {
                                    coordinate: Coordinate {
                                        x: (pixel.coordinate.x + 1),
                                        y: (pixel.coordinate.y),
                                    },
                                    character: 'O',
                                });
                            }
                        }
                    }
                }
            }
            _ => {} // Do nothing for invalid moves
        }

        // Place the cursor back in the grid at the new position
        self.next_frame.push(Pixel {
            coordinate: Coordinate {
                x: (self.cursor_position.x),
                y: (self.cursor_position.y),
            },
            character: '@',
        });

        self.render_grid();
    }

    fn render_grid(&mut self) {
        let mut stdout = stdout();

        if self.next_frame.is_empty() {
            // Print the grid for the first time
            // Clear the terminal
            execute!(stdout, Clear(ClearType::All)).expect("Failed to clear screen");
            // Move cursor to the top-left corner
            execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

            for row in &self.grid {
                for &cell in row {
                    print!("{}", cell);
                }
                println!();
            }
        } else {
            // Update only the pixels that have changed
            for pixel in &self.next_frame {
                self.grid[pixel.coordinate.y][pixel.coordinate.x] = pixel.character;

                execute!(
                    stdout,
                    cursor::MoveTo(pixel.coordinate.x as u16, pixel.coordinate.y as u16),
                )
                .unwrap();

                execute!(stdout, Print(pixel.character)).unwrap();
            }
        }

        execute!(stdout, cursor::MoveTo(0, (self.grid_height + 1) as u16)).unwrap();

        println!("Cursor: {:?}", self.cursor_position);
        self.update_info();

        stdout.flush().unwrap();
    }

    fn update_info(&self) {
        let mut stdout = stdout();

        // Move cursor to the specified row (row is zero-indexed)
        execute!(stdout, cursor::MoveTo(0, (self.grid_height + 3) as u16)).unwrap();

        // Clear the current line (this will clear the specific row)
        execute!(stdout, terminal::Clear(ClearType::FromCursorDown)).unwrap();

        stdout.flush().unwrap();

        println!("Info: {}", self.info);

        stdout.flush().unwrap();
    }
}

fn transform(input: &str) -> Lanternfish {
    // Destructure the split result into two parts
    let (grid_part, input_part) = input
        .trim()
        .split_once("\n\n")
        .expect("Input must contain a grid and an input separated by a blank line");

    // Convert the grid into a Vec<Vec<char>>
    let grid = grid_part
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Convert the remaining input into a Vec<char>
    let input = input_part.chars().collect();

    Lanternfish::new(grid, input)
}

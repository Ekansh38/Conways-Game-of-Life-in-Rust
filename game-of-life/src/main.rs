use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let size = 10.0;

    let mut width_of_screen = screen_width();
    let mut height_of_screen = screen_height();

    let result_of_function = calculate_rows_and_cols(size);
    let mut rows = result_of_function.0;
    let mut cols = result_of_function.1;

    let mut grid = make_2d_array(rows, cols);

    loop {
        clear_background(BLACK);

        if screen_width() != width_of_screen || screen_height() != height_of_screen {
            width_of_screen = screen_width();
            height_of_screen = screen_height();
            let result_of_function = calculate_rows_and_cols(size);
            rows = result_of_function.0;
            cols = result_of_function.1;

            if grid.len() < rows as usize {
                grid.resize_with(rows as usize, || Vec::new());
            } else if grid.len() > rows as usize {
                grid.truncate(rows as usize);
            }

            for row in grid.iter_mut() {
                if row.len() != cols as usize {
                    row.resize(cols as usize, false);
                }
            }
        }

        // Update grid
        if is_key_down(KeyCode::Space) {
            let mut new_grid = grid.clone();
            for row in 0..rows {
                for col in 0..cols {
                    let mut neighbors = 0;
                    let dirs = [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ];

                    for dir in dirs.iter() {
                        let new_row = row + dir.0;
                        let new_col = col + dir.1;

                        // wrap around
                        let new_row = if new_row < 0 {
                            rows - 1
                        } else if new_row >= rows {
                            0
                        } else {
                            new_row
                        };

                        let new_col = if new_col < 0 {
                            cols - 1
                        } else if new_col >= cols {
                            0
                        } else {
                            new_col
                        };

                        if grid[new_row as usize][new_col as usize] {
                            neighbors += 1;
                        }
                    }

                    if grid[row as usize][col as usize] {
                        if neighbors < 2 || neighbors > 3 {
                            new_grid[row as usize][col as usize] = false;
                        }
                    } else {
                        if neighbors == 3 {
                            new_grid[row as usize][col as usize] = true;
                        }
                    }
                }
            }
            grid = new_grid.clone();
        }

        let mouse_position = mouse_position();
        let mouse_position = (mouse_position.0 / size, mouse_position.1 / size);
        let mouse_position = (mouse_position.0 as i32, mouse_position.1 as i32);

        for row in 0..rows {
            for col in 0..cols {
                if mouse_position == (col, row) {
                    if is_mouse_button_down(MouseButton::Left) {
                        grid[row as usize][col as usize] = true;
                    } else if is_mouse_button_down(MouseButton::Right) {
                        grid[row as usize][col as usize] = false;
                    }
                }
            }
        }

        // Draw grid

        for row in 0..rows {
            for col in 0..cols {
                let x = col as f32 * size;
                let y = row as f32 * size;

                if grid[row as usize][col as usize] {
                    draw_rectangle(x, y, size, size, WHITE);
                }
            }
        }

        next_frame().await
    }
}

fn calculate_rows_and_cols(size: f32) -> (i32, i32) {
    let rows = screen_height() / size;
    let rows: i32 = rows as i32;

    let cols = screen_width() / size;
    let cols: i32 = cols as i32;

    (rows, cols)
}

fn make_2d_array(rows: i32, cols: i32) -> Vec<Vec<bool>> {
    let mut grid = vec![];
    for row in 0..rows {
        grid.push(vec![]);
        for _col in 0..cols {
            grid[row as usize].push(false)
        }
    }

    grid
}

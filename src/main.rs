mod part;

use crate::part::direction::Direction;
use crate::part::Orientation;
use crate::part::Part;

#[derive(Clone, Debug)]
struct Node {
    occupied: bool,
    part: Option<Part>,
}

struct Gameboard {
    board: Vec<Vec<Node>>,
    current: (usize, usize),
    next: (usize, usize),
}

impl Gameboard {
    fn new(x: usize, y: usize) -> Gameboard {
        let mut board = Vec::new();
        for i in 0..9 {
            let row_length;
            if i > 4 {
                row_length = 8 - i + 5;
            } else {
                row_length = i + 5;
            }
            println!("row_length = {}", row_length);
            let mut row = Vec::new();
            for _ in 0..row_length {
                row.push(Node {
                    occupied: false,
                    part: None,
                });
            }
            board.push(row);
        }
        board[y][x].occupied = true;
        Gameboard {
            board: board,
            current: (0, 0),
            next: (0, 0),
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.board {
            for node in row {
                if node.occupied {
                    result.push_str("X ");
                } else {
                    result.push_str("O ");
                }
            }
            result.push_str("\n");
        }
        result
    }

    fn is_point_in_bounds(x: i32, y: i32) -> Option<(usize, usize)> {
        if x < 0 || y < 0 {
            return None;
        }

        if y > 8 {
            return None;
        }

        if y < 5 {
            if x > y + 4 {
                return None;
            }
        } else {
            if x > 12 - y {
                return None;
            }
        }

        Some((x as usize, y as usize))
    }

    fn get_node(&self, x: usize, y: usize) -> &Node {
        &self.board[y][x]
    }

    fn get_top_left(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::UpLeft) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_top_right(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::UpRight) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_left(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::Left) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_right(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::Right) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_bottom_left(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::DownLeft) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_bottom_right(&self, x: i32, y: i32) -> Option<&Node> {
        match Self::direction_to_board_coords(x, y, &Direction::DownRight) {
            None => None,
            Some((x, y)) => Some(&self.board[y][x]),
        }
    }

    fn get_next_free_field(&self, start_x: usize, start_y: usize) -> Option<(usize, usize)> {
        let mut x = start_x;
        let mut y = start_y;

        loop {
            if !self.board[y][x].occupied {
                break;
            }

            if y < 5 {
                if x == y + 4 {
                    x = 0;
                    y += 1;
                } else {
                    x += 1;
                }
            } else {
                if x == 12 - y {
                    x = 0;
                    y += 1;
                } else {
                    x += 1;
                }
            }

            if y >= 9 {
                return None;
            }
        }

        Some((x, y))
    }

    fn is_field_free(&self, x: usize, y: usize) -> bool {
        !self.board[y][x].occupied
    }

    fn directions_fit(&self, x: usize, y: usize, directions: &Vec<Vec<Direction>>) -> bool {
        for direction in directions {
            let mut current_x = x;
            let mut current_y = y;
            for step in direction {
                let (new_x, new_y);
                match Self::direction_to_board_coords(current_x as i32, current_y as i32, step) {
                    None => return false,
                    Some((new_x_value, new_y_value)) => {
                        new_x = new_x_value;
                        new_y = new_y_value;
                    }
                }
                if self.get_node(new_x, new_y).occupied {
                    return false;
                }
                current_x = new_x;
                current_y = new_y;
            }
        }

        true
    }

    fn direction_to_board_coords(x: i32, y: i32, direction: &Direction) -> Option<(usize, usize)> {
        let (new_x, new_y) = match direction {
            Direction::UpLeft => {
                if y < 4 {
                    (x - 1, y - 1)
                } else {
                    (x, y - 1)
                }
            }
            Direction::UpRight => {
                if y < 4 {
                    (x, y - 1)
                } else {
                    (x + 1, y - 1)
                }
            }
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::DownRight => {
                if y < 4 {
                    (x + 1, y + 1)
                } else {
                    (x, y + 1)
                }
            }
            Direction::DownLeft => {
                if y < 4 {
                    (x, y + 1)
                } else {
                    (x - 1, y + 1)
                }
            }
        };

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((new_x, new_y)) => Some((new_x as usize, new_y as usize)),
        }
    }

    fn place_part(&mut self, x: usize, y: usize, directions: &Vec<Vec<Direction>>) {
        for direction in directions {
            let mut current_x = x;
            let mut current_y = y;

            self.board[current_y][current_x].occupied = true;
            for step in direction {
                let new_coords_option =
                    Self::direction_to_board_coords(current_x as i32, current_y as i32, step);
                let new_coords;
                match new_coords_option {
                    Some(new_coords_value) => new_coords = new_coords_value,
                    None => panic!("Trying to place part at invalid location"),
                }

                self.board[new_coords.1][new_coords.0].occupied = true;
                current_x = new_coords.0;
                current_y = new_coords.1;
            }
            self.board[current_y][current_x].occupied = true;
        }
    }

    fn remove_part(&mut self, x: usize, y: usize, directions: &Vec<Vec<Direction>>) {
        self.board[y][x].occupied = false;
        for direction in directions {
            let mut current_x = x;
            let mut current_y = y;
            for step in direction {
                let new_coords_option =
                    Self::direction_to_board_coords(current_x as i32, current_y as i32, step);
                let new_coords;
                match new_coords_option {
                    Some(new_coords_value) => new_coords = new_coords_value,
                    None => panic!("Trying to remove part at invalid location"),
                }

                self.board[new_coords.1][new_coords.0].occupied = false;
                current_x = new_coords.0;
                current_y = new_coords.1;
            }
        }
    }
}

fn main() {
    let mut gameboard = Gameboard::new(4, 4);
    let parts = Part::get_parts_list();
    let mut current_field: (usize, usize);
    let mut result_list: Vec<(usize, Orientation, usize, usize)> = Vec::new();
    let mut deleted_part: Option<(usize, Orientation, usize, usize)> = None;
    let mut counter = 0;
    let mut result_counter = 0;

    loop {
        // println!("Gameboard:\n{}", gameboard.to_string());
        println!("Result list: {:?}", result_list);
        // println!("Counter: {}", counter);
        counter += 1;

        if result_list.len() == 12 {
            println!("Solution found!");
            // break;
            result_counter += 1;
            deleted_part = result_list.pop();
            match deleted_part {
                None => {
                    println!("No part to delete found");
                    println!("No more solutions found?");
                    break;
                }
                Some(deleted_part_value) => {
                    println!("Deleting part {}", deleted_part_value.0);
                    gameboard.remove_part(
                        deleted_part_value.2,
                        deleted_part_value.3,
                        &parts[deleted_part_value.0].get_directions(deleted_part_value.1),
                    );
                    continue;
                }
            }
        }
        // get next field to check
        let current_field_option = gameboard.get_next_free_field(0, 0);
        match current_field_option {
            None => {
                println!("No free field found");
                println!("Result list: {:?}", result_list);
                println!("Found Solution?");
                break;
            }
            Some((x, y)) => {
                // println!("Found free field at ({}, {})", x, y);
                current_field = (x, y);
            }
        }

        // check parts if they fit
        let mut found_part = false;

        // start with deleted part if one is found, if deleted par has no next part, pop from result list (two last where wrong)
        let mut current_part;
        match deleted_part {
            Some(deleted_part_value) => {
                let current_part_option =
                    Part::get_next_part(deleted_part_value.0, deleted_part_value.1);
                match current_part_option {
                    None => {
                        deleted_part = result_list.pop();
                        match deleted_part {
                            None => {
                                println!("No part to delete found");
                                println!("No solution found?");
                                break;
                            }
                            Some(deleted_part_value) => {
                                // println!("Deleting part {}", deleted_part_value.0);
                                gameboard.remove_part(
                                    deleted_part_value.2,
                                    deleted_part_value.3,
                                    &parts[deleted_part_value.0]
                                        .get_directions(deleted_part_value.1),
                                );
                                continue;
                            }
                        }
                    }
                    Some(current_part_value) => {
                        current_part = current_part_value;
                    }
                }
                deleted_part = None;
            }
            None => current_part = (0, Orientation::A),
        }

        // check if part fits and is not used yet
        loop {
            let part_id = current_part.0;
            let orientation = current_part.1;

            let part_already_used = result_list.iter().any(|&x| x.0 == part_id);

            if part_already_used {
                // println!("Part {} already used", part_id);
                let next_part_option = Part::get_next_part(part_id, orientation);
                match next_part_option {
                    None => {
                        // println!("No next part found");
                        deleted_part = result_list.pop();
                        match deleted_part {
                            None => {
                                // println!("No part to delete found");
                                // println!("No solution found?");
                                break;
                            }
                            Some(deleted_part_value) => {
                                // println!("Deleting part {}", deleted_part_value.0);
                                gameboard.remove_part(
                                    deleted_part_value.2,
                                    deleted_part_value.3,
                                    &parts[deleted_part_value.0]
                                        .get_directions(deleted_part_value.1),
                                );
                                break;
                            }
                        }
                    }
                    Some(next_part) => {
                        current_part = next_part;
                        continue;
                    }
                }
            }

            let directions = parts[part_id].get_directions(orientation);
            found_part = gameboard.directions_fit(current_field.0, current_field.1, &directions);

            if found_part {
                // println!(
                //     "Found part {} at ({}, {})",
                //     part_id, current_field.0, current_field.1
                // );
                gameboard.place_part(current_field.0, current_field.1, &directions);
                result_list.push((part_id, orientation, current_field.0, current_field.1));
                break;
            } else {
                // println!("Part {} does not fit", part_id);
                let next_part_option = Part::get_next_part(part_id, orientation);
                match next_part_option {
                    None => {
                        // println!("No next part found");
                        deleted_part = result_list.pop();
                        match deleted_part {
                            None => {
                                println!("No part to delete found");
                                println!("No solution found?");
                                break;
                            }
                            Some(deleted_part_value) => {
                                // println!("Deleting part {}", deleted_part_value.0);
                                gameboard.remove_part(
                                    deleted_part_value.2,
                                    deleted_part_value.3,
                                    &parts[deleted_part_value.0]
                                        .get_directions(deleted_part_value.1),
                                );
                                break;
                            }
                        }
                    }
                    Some(next_part) => {
                        current_part = next_part;
                        continue;
                    }
                }
            }
        }
    }

    println!("Result counter: {}", result_counter)
}

// Result list: []
// Counter: 685140
// Found free field at (0, 0)
// No part to delete found
// No solution found?
// Result counter: 2

// Result list: []
// Counter: 290994
// Found free field at (0, 0)
// No part to delete found
// No solution found?
// Result counter: 5
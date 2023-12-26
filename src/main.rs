mod part;

use crate::part::Part;

#[derive(Clone, Debug)]
struct Node {
    occupied: bool,
    part: Option<Part>,
}

struct Gameboard {
    board: Vec<Vec<Node>>,
}

impl Gameboard {
    fn new() -> Gameboard {
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
        Gameboard { board: board }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.board {
            for node in row {
                if node.occupied {
                    result.push_str("X");
                } else {
                    result.push_str("O");
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

        if x > 8 {
            return None;
        }

        if x < 5 {
            if y > x + 4 {
                return None;
            }
        } else {
            if y > 12 - x {
                return None;
            }
        }

        Some((x as usize, y as usize))
    }

    fn get_top_left(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x - 1;
        let new_y = y - 1;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }

    fn get_top_right(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x;
        let new_y = y - 1;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }

    fn get_left(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x - 1;
        let new_y = y;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }

    fn get_right(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x + 1;
        let new_y = y;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }

    fn get_bottom_left(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x;
        let new_y = y + 1;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }

    fn get_bottom_right(&self, x: i32, y: i32) -> Option<&Node> {
        let new_x = x + 1;
        let new_y = y + 1;

        match Self::is_point_in_bounds(new_x, new_y) {
            None => None,
            Some((x, y)) => Some(&self.board[x][y]),
        }
    }
}
fn main() {
    let gameboard = Gameboard::new();
    match gameboard.get_right(0, 0) {
        None => println!("None"),
        Some(node) => println!("{:?}", node),
    }
}

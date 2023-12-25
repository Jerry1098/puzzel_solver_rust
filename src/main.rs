

#[derive(Clone)]
enum Direction {
    UpLeft,
    UpRight,
    Right,
    Left,
    DownRight,
    DownLeft
}

#[derive(Clone)]
struct Part {
    directions_a: Vec<Vec<Direction>>,
    directions_b: Vec<Vec<Direction>>,
    directions_c: Vec<Vec<Direction>>,
    id: i32
}

impl Part {
    fn get_parts_list() -> Vec<Part> {
        let mut parts = Vec::new();

        parts.push(Part {
            directions_a: vec![vec![Direction::Right, Direction::DownLeft, Direction::Left, Direction::Left]],
            directions_b: vec![vec![Direction::DownLeft, Direction::DownLeft, Direction::Right, Direction::DownLeft]],
            directions_c: vec![vec![Direction::DownRight, Direction::UpRight, Direction::DownRight, Direction::DownRight]],
            id: 1
        });

        parts.push(Part {
            directions_a: vec![vec![Direction::DownLeft, Direction::DownLeft], vec![Direction::Right, Direction::Right]],
            directions_b: vec![vec![Direction::DownRight, Direction::DownRight, Direction::DownLeft, Direction::DownLeft]],
            directions_c: vec![vec![Direction::DownRight, Direction::DownRight, Direction::Right, Direction::Right]],
            id: 2
        });

        parts.push(Part {
            directions_a: vec![vec![Direction::DownLeft, Direction::Left, Direction::Left], vec![Direction::Right]],
            directions_b: vec![vec![Direction::DownLeft, Direction::DownLeft, Direction::DownRight, Direction::DownLeft]],
            directions_c: vec![vec![Direction::DownRight, Direction::Right, Direction::DownRight, Direction::DownRight]],
            id: 3
        });
        
        parts
    }
}

#[derive(Clone)]
struct Node {
    occupied: bool,
    part: Option<Part>,
}

struct Gameboard {
    board: Vec<Vec<Node>>
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
                    part: None
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

    fn top_left(&self, x: usize, y: usize) -> Option<&Node> {
        if x == 0 || y == 0 {
            None
        } else {
            Some(&self.board[x - 1][y - 1])
        }
    }
}

fn main() {
    let gameboard = Gameboard::new();
    
}

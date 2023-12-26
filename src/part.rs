mod direction;

use crate::part::direction::Direction;
use gnuplot::{AxesCommon, Caption, Color, Figure, PlotOption::PointSize, AutoOption::Fix};

#[derive(Clone, Debug)]
pub struct Part {
    pub directions_a: Vec<Vec<Direction>>,
    pub directions_b: Vec<Vec<Direction>>,
    pub directions_c: Vec<Vec<Direction>>,
    pub id: i32,
}

impl Part {
    pub fn get_parts_list() -> Vec<Part> {
        let mut parts = Vec::new();

        parts.push(Part {
            directions_a: vec![vec![
                Direction::Right,
                Direction::DownLeft,
                Direction::Left,
                Direction::Left,
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::Right,
                Direction::DownLeft,
            ]],
            directions_c: vec![vec![
                Direction::DownRight,
                Direction::UpRight,
                Direction::DownRight,
                Direction::DownRight,
            ]],
            id: 1,
        });

        parts.push(Part {
            directions_a: vec![
                vec![Direction::DownLeft, Direction::DownLeft],
                vec![Direction::Right, Direction::Right],
            ],
            directions_b: vec![vec![
                Direction::DownRight,
                Direction::DownRight,
                Direction::DownLeft,
                Direction::DownLeft,
            ]],
            directions_c: vec![vec![
                Direction::DownRight,
                Direction::DownRight,
                Direction::Right,
                Direction::Right,
            ]],
            id: 2,
        });

        parts.push(Part {
            directions_a: vec![
                vec![Direction::DownLeft, Direction::Left, Direction::Left],
                vec![Direction::Right],
            ],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::DownRight,
                Direction::DownLeft,
            ]],
            directions_c: vec![vec![
                Direction::DownRight,
                Direction::Right,
                Direction::DownRight,
                Direction::DownRight,
            ]],
            id: 3,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownRight,
                Direction::Left,
                Direction::DownLeft,
                Direction::Left,
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownRight,
                Direction::DownRight,
                Direction::Left,
            ]],
            directions_c: vec![vec![
                Direction::DownLeft,
                Direction::Right,
                Direction::Right,
                Direction::DownRight,
            ]],
            id: 4,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::Right,
            ], vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::UpRight,
            ]],
            directions_b: vec![vec![
                Direction::Right,
                Direction::DownLeft,
                Direction::DownRight,
                Direction::DownLeft,
            ]],
            directions_c: vec![vec![
                Direction::DownRight,
                Direction::Right,
                Direction::DownRight,
                Direction::UpRight
            ]],
            id: 5,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::UpLeft,
                Direction::Left,
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::Left,
                Direction::DownRight,
                Direction::DownRight
            ]],

            directions_c: vec![vec![
                Direction::Right,
                Direction::Right,
                Direction::DownLeft,
                Direction::DownRight
            ]],
            id: 6,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownRight,
                Direction::Right,
                Direction::UpRight,
                Direction::DownRight
            ]],
            directions_b: vec![vec![
                Direction::Right,
            ], vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::Right,
            ]],

            directions_c: vec![vec![
                Direction::DownLeft,
                Direction::Right,
                Direction::DownRight,
                Direction::DownLeft
            ]],
            id: 7,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownLeft,
                Direction::Right,
                Direction::UpRight,
                Direction::Right,
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::Right,
                Direction::DownLeft,
                Direction::DownLeft,
            ]],

            directions_c: vec![vec![
                Direction::DownRight,
                Direction::DownRight,
                Direction::UpRight,
                Direction::DownRight
            ]],
            id: 8,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::Right,
                Direction::Right,
                Direction::DownRight,
                Direction::UpRight
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::UpLeft,
            ]],

            directions_c: vec![vec![
                Direction::Right,
                Direction::DownLeft,
                Direction::DownRight,
                Direction::DownRight
            ]],
            id: 9,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownRight,
                Direction::DownLeft,
                Direction::Right,
                Direction::Right,
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft
            ],vec![
                Direction::DownRight,
                Direction::Right
            ]],

            directions_c: vec![vec![
                Direction::DownRight,
                Direction::DownRight,
                Direction::Left,
                Direction::DownLeft
            ]],
            id: 10,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownLeft,
                Direction::Left,
                Direction::Left
            ], vec![
                Direction::DownLeft,
                Direction::DownRight
            ]],
            directions_b: vec![vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::Left,
            ], vec![
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::DownRight
            ]],

            directions_c: vec![vec![
                Direction::DownLeft,
                Direction::Left,
            ], vec![
                Direction::DownLeft,
                Direction::DownRight,
                Direction::DownRight
            ]],
            id: 11,
        });

        parts.push(Part {
            directions_a: vec![vec![
                Direction::DownLeft
            ], vec![
                Direction::Right,
                Direction::Right,
                Direction::Right
            ]],
            directions_b: vec![vec![
                Direction::DownRight,
                Direction::DownLeft,
                Direction::DownLeft,
                Direction::DownLeft
            ]],

            directions_c: vec![vec![
                Direction::DownRight,
                Direction::DownRight,
                Direction::DownRight,
                Direction::Left
            ]],
            id: 12,
        });

        parts
    }

    pub fn visualize_part(directions: &Vec<Vec<Direction>>) {
        // calculate size of shape
        let mut x = 0.;
        let mut y = 0.;

        let mut x_min = 0.;
        let mut x_max = 0.;
        let mut y_min = 0.;
        let mut y_max = 0.;

        let mut point_locations = vec![(0.,0.)];

        for direction in directions {
            for dir in direction {
                match dir {
                    Direction::UpLeft => {
                        x -= 1.;
                        y += 1.;
                    }
                    Direction::UpRight => {
                        x += 1.;
                        y += 1.;
                    }
                    Direction::Right => {
                        x += 2.;
                    }
                    Direction::Left => {
                        x -= 2.;
                    }
                    Direction::DownRight => {
                        x += 1.;
                        y -= 1.;
                    }
                    Direction::DownLeft => {
                        x -= 1.;
                        y -= 1.;
                    }
                }

                point_locations.push((x, y));

                if x < x_min {
                    x_min = x;
                }
                if x > x_max {
                    x_max = x;
                }
                if y < y_min {
                    y_min = y;
                }
                if y > y_max {
                    y_max = y;
                }
            }
            x = 0.;
            y = 0.;
        }

        println!("x = {}, y = {}", x, y);
        println!("x_min = {}, x_max = {}", x_min, x_max);
        println!("y_min = {}, y_max = {}", y_min, y_max);
        println!("point_locations = {:?}", point_locations);

        let mut fg = Figure::new();
        {
            let axes2d = fg.axes2d();
            axes2d.set_x_range(Fix(x_min - 0.5), Fix(x_max + 0.5));
            axes2d.set_y_range(Fix(y_min - 0.5), Fix(y_max + 0.5));
            let (x, y): (Vec<_>, Vec<_>) = point_locations.iter().cloned().unzip();
            axes2d.points(&x, &y, &[Caption("Points"), Color("blue"), PointSize(2.0)]);
            axes2d.lines(&x, &y, &[Caption("Lines"), Color("red")]);
        }
        fg.show().unwrap();
    }
}
use std::boxed::Box;

enum Directions {
    Right,
    Left,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft
}

#[derive(Clone)]
enum NodeAddress {
    Address(Box<Node>),
    Nil
}

impl NodeAddress {
    fn add(self, append_node: Node, direction: Directions) -> bool{
        match self {
            NodeAddress::Address(mut node) => {
                let mut_node = node.as_mut();
                let new_node = NodeAddress::Address(Box::new(append_node));

                match direction {
                    Directions::Right => {mut_node.right = new_node},
                    Directions::Left => {mut_node.left = new_node},
                    Directions::TopRight => {mut_node.top_right = new_node},
                    Directions::TopLeft => {mut_node.top_left = new_node},
                    Directions::BottomRight => {mut_node.bottom_right = new_node},
                    Directions::BottomLeft => {mut_node.bottom_left = new_node},
                }

                true
            }, 
            Self::Nil =>{
                false
            }
        }
    }

    fn get(self, direction: Directions) -> Option<NodeAddress> {
        match self {
            NodeAddress::Address(node) => {

                match direction {
                    Directions::Right => {Some(node.right)},
                    Directions::Left => {Some(node.left)},
                    Directions::TopRight => {Some(node.top_right)},
                    Directions::TopLeft => {Some(node.top_left)},
                    Directions::BottomRight => {Some(node.bottom_right)},
                    Directions::BottomLeft => {Some(node.bottom_left)},
                }

            },
            NodeAddress::Nil => None,
        }
    }
}


#[derive(Clone)]
struct Node {
    occupied: bool,
    part: Option<Part>,
    right: NodeAddress,
    left: NodeAddress,
    top_right: NodeAddress,
    top_left: NodeAddress,
    bottom_right: NodeAddress,
    bottom_left: NodeAddress
}

impl Node {
    fn new_empty() -> Node {
        Node {
            occupied: false,
            part: None,
            right: NodeAddress::Nil,
            left: NodeAddress::Nil,
            top_right: NodeAddress::Nil,
            top_left: NodeAddress::Nil,
            bottom_right: NodeAddress::Nil,
            bottom_left: NodeAddress::Nil
        }
    }
}

#[derive(Clone)]
enum PartOrientation {
    A,
    B,
    C
}


#[derive(Clone)]
struct Part {
    id: i32,
    orientation: PartOrientation
}

struct Gameboard {
    start_node: NodeAddress
}

impl Gameboard {
    fn new() -> Gameboard {
        let node = Node::new_empty();
        let mut gameboard = Gameboard {
            start_node: NodeAddress::Address(Box::new(node))
        };

        let ref mut current_main_node = gameboard.start_node;

        for i in 1..5 {
            let side_nodes_count = i + 4;
            for j in 1..side_nodes_count {
                match current_main_node {
                    NodeAddress::Address(ref mut node) => {
                        let new_node = Node::new_empty();
                        let new_node_address = NodeAddress::Address(Box::new(new_node));

                        node.right = new_node_address;
                        current_main_node = node.right.clone();
                    },
                    NodeAddress::Nil => {}
                }
            }
        }

        gameboard
    }
}
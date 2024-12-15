use crate::RobotMoveCommand::{Down, Left, Right, Up};
use crate::WarehouseObject::*;
use crate::WideWarehouseObject::BoxLeft;
use aoc_util::Coordinate;
use hashbrown::HashMap;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WarehouseObject {
    Box,
    Block,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WideWarehouseObject {
    BoxLeft,
    BoxRight,
    Block,
}

#[derive(Debug, Copy, Clone)]
enum RobotMoveCommand {
    Up,
    Down,
    Left,
    Right,
}

impl RobotMoveCommand {
    fn get_direction_vec(&self) -> Coordinate {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
        .into()
    }
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let mut warehouse_layout = HashMap::new();
    let mut wide_warehouse_layout = HashMap::new();

    let mut robot_pos: Option<Coordinate> = None;
    let mut robot_wide_pos: Option<Coordinate> = None;
    let mut commands = Vec::new();
    let mut read_warehouse = true;

    for (ver_pos, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            read_warehouse = false;
            continue;
        }
        if read_warehouse {
            for (hor_pos, object) in line.char_indices() {
                match object {
                    '#' => {
                        warehouse_layout.insert(Coordinate::new_u(ver_pos, hor_pos), Block);
                        wide_warehouse_layout.insert(
                            Coordinate::new_u(ver_pos, hor_pos * 2),
                            WideWarehouseObject::Block,
                        );
                        wide_warehouse_layout.insert(
                            Coordinate::new_u(ver_pos, hor_pos * 2 + 1),
                            WideWarehouseObject::Block,
                        );
                    }
                    'O' => {
                        warehouse_layout.insert(Coordinate::new_u(ver_pos, hor_pos), Box);
                        wide_warehouse_layout.insert(
                            Coordinate::new_u(ver_pos, hor_pos * 2),
                            WideWarehouseObject::BoxLeft,
                        );
                        wide_warehouse_layout.insert(
                            Coordinate::new_u(ver_pos, hor_pos * 2 + 1),
                            WideWarehouseObject::BoxRight,
                        );
                    }
                    '@' => {
                        robot_pos = Some(Coordinate::new_u(ver_pos, hor_pos));
                        robot_wide_pos = Some(Coordinate::new_u(ver_pos, hor_pos * 2));
                    }
                    _ => (),
                }
            }
        } else {
            for command_raw in line.chars() {
                let command = match command_raw {
                    '^' => Up,
                    'v' => Down,
                    '<' => Left,
                    '>' => Right,
                    _ => unreachable!(),
                };
                commands.push(command);
            }
        }
    }

    let mut robot_pos = robot_pos.unwrap();
    let mut robot_wide_pos = robot_wide_pos.unwrap();

    for command in commands {
        try_perform_command(&mut robot_pos, command, &mut warehouse_layout);
        try_perform_wide_command(&mut robot_wide_pos, command, &mut wide_warehouse_layout);
    }

    let res_p1 = warehouse_layout
        .iter()
        .filter(|(_, v)| **v == Box)
        .map(|(coord, _)| coord.ver_idx * 100 + coord.hor_idx)
        .sum::<isize>();

    let res_p2 = wide_warehouse_layout
        .iter()
        .filter(|(_, v)| **v == BoxLeft)
        .map(|(coord, _)| coord.ver_idx * 100 + coord.hor_idx)
        .sum::<isize>();

    (res_p1 as usize, res_p2 as usize)
}

fn print_map(
    robot_pos: Coordinate,
    warehouse_layout: &HashMap<Coordinate, WideWarehouseObject>,
    command: RobotMoveCommand,
) {
    println!("Move {command:?}");
    use WideWarehouseObject::*;
    for ver_idx in 0..8 {
        for hor_idx in 0..16 {
            if robot_pos == Coordinate::new_u(ver_idx, hor_idx) {
                print!("@");
            } else {
                match warehouse_layout.get(&Coordinate::new_u(ver_idx, hor_idx)) {
                    None => print!("."),
                    Some(BoxLeft) => print!("["),
                    Some(BoxRight) => print!("]"),
                    Some(Block) => print!("#"),
                }
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn try_perform_wide_command(
    robot_pos: &mut Coordinate,
    command: RobotMoveCommand,
    warehouse_layout: &mut HashMap<Coordinate, WideWarehouseObject>,
) {
    if let Some(followers) = wide_boxes_to_move(robot_pos, command, warehouse_layout) {
        let direction = command.get_direction_vec();

        for (box_left, box_right) in followers.iter().rev() {
            warehouse_layout.remove(box_left);
            warehouse_layout.remove(box_right);

            warehouse_layout.insert(*box_left + direction, WideWarehouseObject::BoxLeft);
            warehouse_layout.insert(*box_right + direction, WideWarehouseObject::BoxRight);
        }

        *robot_pos = *robot_pos + direction;
    }
}

fn wide_boxes_to_move(
    initial_pos: &Coordinate,
    command: RobotMoveCommand,
    warehouse_layout: &mut HashMap<Coordinate, WideWarehouseObject>,
) -> Option<Vec<(Coordinate, Coordinate)>> {
    use WideWarehouseObject::*;

    let direction = command.get_direction_vec();

    let mut push_layer = HashSet::new();
    push_layer.insert(*initial_pos + direction);
    let mut next_push_layer = HashSet::new();
    let mut boxes_to_move = Vec::new();

    while !push_layer.is_empty() {
        for pos_at in push_layer {
            let mut pos_at = pos_at;
            if let Some(warehouse_object) = warehouse_layout.get(&pos_at) {
                if *warehouse_object == Block {
                    return None;
                }

                match (*warehouse_object, command) {
                    (BoxLeft, Up) | (BoxLeft, Down) => {
                        next_push_layer.insert(pos_at + direction);
                        next_push_layer.insert(pos_at + Right.get_direction_vec() + direction);
                        boxes_to_move.push((pos_at, pos_at + Right.get_direction_vec()));
                    }
                    (BoxRight, Up) | (BoxRight, Down) => {
                        next_push_layer.insert(pos_at + direction);
                        next_push_layer.insert(pos_at + Left.get_direction_vec() + direction);
                        boxes_to_move.push((pos_at + Left.get_direction_vec(), pos_at));
                    }
                    (BoxRight, Left) => {
                        next_push_layer.insert(pos_at + direction + direction);
                        boxes_to_move.push((pos_at + Left.get_direction_vec(), pos_at));
                    }
                    (BoxLeft, Right) => {
                        next_push_layer.insert(pos_at + direction + direction);
                        boxes_to_move.push((pos_at, pos_at + Right.get_direction_vec()));
                    }
                    _ => unreachable!(),
                }
            }
        }
        push_layer = next_push_layer;
        next_push_layer = HashSet::new();
    }

    Some(boxes_to_move)
}

fn try_perform_command(
    robot_pos: &mut Coordinate,
    command: RobotMoveCommand,
    warehouse_layout: &mut HashMap<Coordinate, WarehouseObject>,
) {
    if let Some(followers) = boxes_to_move(robot_pos, command, warehouse_layout) {
        let direction = command.get_direction_vec();

        if followers.len() == 1 {
            warehouse_layout.remove(&followers[0]);
            warehouse_layout.insert(followers[0] + direction, Box);
        } else if followers.len() > 1 {
            warehouse_layout.remove(&followers[0]);
            warehouse_layout.insert(followers[followers.len() - 1] + direction, Box);
        }

        *robot_pos = *robot_pos + direction;
    }
}

fn boxes_to_move(
    initial_pos: &Coordinate,
    command: RobotMoveCommand,
    warehouse_layout: &mut HashMap<Coordinate, WarehouseObject>,
) -> Option<Vec<Coordinate>> {
    let direction = command.get_direction_vec();

    let mut pos_at = *initial_pos + direction;
    let mut boxes_to_move = Vec::new();

    while let Some(warehouse_object) = warehouse_layout.get(&pos_at) {
        if *warehouse_object == Block {
            return None;
        } else {
            boxes_to_move.push(pos_at.clone());
            pos_at = pos_at + direction;
        }
    }

    Some(boxes_to_move)
}

use crate::RobotMoveCommand::{Down, Left, Right, Up};
use crate::WarehouseObject::*;
use aoc_util::Coordinate;
use hashbrown::HashMap;
use std::cmp::PartialEq;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WarehouseObject {
    Box,
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
    let mut robot_pos: Option<Coordinate> = None;
    let mut commands = Vec::new();
    let mut read_warehouse = true;

    for (ver_pos, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            read_warehouse = false;
            break;
        }
        if read_warehouse {
            for (hor_pos, object) in line.char_indices() {
                match object {
                    '#' => {
                        warehouse_layout.insert(Coordinate::new_u(ver_pos, hor_pos), Block);
                    }
                    'O' => {
                        warehouse_layout.insert(Coordinate::new_u(ver_pos, hor_pos), Box);
                    }
                    '@' => robot_pos = Some(Coordinate::new_u(ver_pos, hor_pos)),
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

    for command in commands {
        try_perform_command(&mut robot_pos, command, &mut warehouse_layout);
    }

    let res_p1 = warehouse_layout
        .iter()
        .filter(|(_, v)| **v == Box)
        .map(|(coord, _)| coord.ver_idx * 100 + coord.hor_idx)
        .sum::<isize>();

    (res_p1 as usize, 5)
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

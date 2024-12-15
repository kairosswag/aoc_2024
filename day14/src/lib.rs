use std::cmp::Ordering;
use hashbrown::HashSet;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct GuardRobot {
    x_pos: isize,
    y_pos: isize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Velocity {
    x_vel: isize,
    y_vel: isize,
}

pub fn run<R>(reader: R) -> (usize, usize)
where
    R: BufRead,
{
    let robots = parse_robots(reader);

    let width = 101;
    let height = 103;

    let res_p1 = calc_res(robots, width, height);
    (res_p1, 5)
}

fn calc_res(robots: Vec<(GuardRobot, Velocity)>, width: isize, height: isize) -> usize {
    let width_mid = (width - 1) / 2;
    let height_mid = (height - 1) / 2;

    let mut q_map = HashSet::new();
    let (mut q1_count, mut q2_count, mut q3_count, mut q4_count) = (0, 0, 0, 0);
    for robot in robots {
        let (final_x, final_y) = restrained_mul_100(robot.1, robot.0, width, height);
        q_map.insert((final_y, final_x));


        match (final_x.cmp(&width_mid), final_y.cmp(&height_mid)) {
            (Ordering::Greater, Ordering::Less) => q1_count += 1,
            (Ordering::Greater, Ordering::Greater) => q2_count += 1,
            (Ordering::Less, Ordering::Less) => q3_count += 1,
            (Ordering::Less, Ordering::Greater) => q4_count += 1,
            (Ordering::Equal, _) => (),
            (_, Ordering::Equal) => (),
        }
    }

    (q1_count * q2_count * q3_count * q4_count) as usize
}

fn restrained_mul_100(
    vel: Velocity,
    guard_robot: GuardRobot,
    width: isize,
    height: isize,
) -> (isize, isize) {
    let x_vec = vel.x_vel * 100;
    let final_x = (x_vec + guard_robot.x_pos).rem_euclid(width);

    let y_vec = vel.y_vel * 100;
    let final_y = (y_vec + guard_robot.y_pos).rem_euclid(height);

    (final_x, final_y)
}

fn parse_robots<R>(reader: R) -> Vec<(GuardRobot, Velocity)>
where
    R: BufRead,
{
    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let position_raw = split.next().unwrap();
        let velocity_raw = split.next().unwrap();

        let mut position = position_raw[2..]
            .split(",")
            .map(|val| val.parse::<isize>().unwrap());
        let x_pos = position.next().unwrap();
        let y_pos = position.next().unwrap();

        let mut velocity = velocity_raw[2..]
            .split(",")
            .map(|val| val.parse::<isize>().unwrap());
        let x_vel = velocity.next().unwrap();
        let y_vel = velocity.next().unwrap();

        robots.push((GuardRobot { x_pos, y_pos }, Velocity { x_vel, y_vel }));
    }

    robots
}

#[cfg(test)]
mod tests_day_14 {
    use crate::{calc_res, parse_robots};
    #[test]
    fn test() {
        let test_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let robots = parse_robots(test_input.as_bytes());
        assert_eq!(12, calc_res(robots, 11, 7));
    }
}

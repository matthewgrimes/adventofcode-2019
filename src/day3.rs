use crate::utils::get_lines;
use std::cmp::{max, min};

pub fn day3(file_path: String) {
    let lines = get_lines(file_path);

    let (path0, distances0) = parse_path(&lines[0]);
    let (path1, distances1) = parse_path(&lines[1]);

    let mut intersections: Vec<i32> = Vec::new();
    for i in 0..path0.len() - 1 {
        let node00 = path0[i];
        let node01 = path0[i + 1];
        for j in 0..path1.len() - 1 {
            let node10 = path1[j];
            let node11 = path1[j + 1];

            let current_distance = distances0[..i].iter().sum::<i32>() + distances1[..j].iter().sum::<i32>();
            let mut distance = 0;
            if node00.0 == node01.0 {
                // first edge is vertical
                if node10.1 == node11.1 {
                    // second edge is horizontal
                    distance = check_intersection(&[node10, node11], &[node00, node01]);
                }
            } else {
                // first edge is horizontal
                if node10.0 == node11.0 {
                    // second edge is vertical
                    distance = check_intersection(&[node00, node01], &[node10, node11]);
                }
            }
            if distance > 0 {
                intersections.push(current_distance + distance);
            }
        }
    }
    println!(
        "{:?}",
        intersections
            .iter()
            .min()
            .unwrap()
    );
}

fn check_intersection(horizontal: &[(i32, i32); 2], vertical: &[(i32, i32); 2]) -> i32 {
    let left = min(horizontal[0].0, horizontal[1].0);
    let right = max(horizontal[0].0, horizontal[1].0);
    let bottom = min(vertical[0].1, vertical[1].1);
    let top = max(vertical[0].1, vertical[1].1);
    // ignore origin
    if horizontal[0].1==0 && vertical[0].0==0 {
        return 0;
    }
    if vertical[0].0 <= right
        && vertical[0].0 >= left
        && horizontal[0].1 <= top
        && horizontal[0].1 >= bottom
    {
        //println!("Horizontal edge:{:?} Vertical edge:{:?}", horizontal, vertical);
        //println!("Intersection:{:?}", (vertical[0].0, horizontal[0].1));
        //println!("Distance:{:?}", horizontal[0].1.abs() + vertical[0].0.abs());
        //println!("Length:{:?}", (horizontal[0].1-vertical[0].1).abs() + (vertical[0].0 - horizontal[0].0).abs());
        //println!("---");
        return (horizontal[0].1-vertical[0].1).abs() + (vertical[0].0 - horizontal[0].0).abs();
    }
    0
}

fn parse_path(path: &str) -> (Vec<(i32, i32)>, Vec<i32>) {
    let steps: Vec<String> = path.split(',').map(|s| s.parse().unwrap()).collect();
    let distances = get_distances(&steps);
    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push((0, 0));
    for step in steps {
        let current_coord = coords.last().unwrap();
        coords.push(get_new_coord(current_coord, step));
    }
    (coords, distances)
}

fn get_distances(steps: &Vec<String>) -> Vec<i32> {
   steps.iter().map(|x| x[1..].parse().unwrap()).collect() 
}

fn get_new_coord(current_coord: &(i32, i32), step: String) -> (i32, i32) {
    let (x, y) = current_coord;
    let direction = &step[..1];
    let distance: i32 = (step[1..]).parse().unwrap();
    match direction {
        "R" => (*x + distance, *y),
        "L" => (*x - distance, *y),
        "U" => (*x, *y + distance),
        "D" => (*x, *y - distance),
        &_ => todo!(),
    }
}

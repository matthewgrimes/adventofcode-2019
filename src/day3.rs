use crate::utils::get_lines;

pub fn day3(file_path: String) {
    let lines = get_lines(file_path);
    let path0 = parse_path(&lines[0]);
    let path1 = parse_path(&lines[1]);
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    for coord0 in &path0 {
        for coord1 in &path1 {
            if coord0 == coord1 {
                intersections.push(*coord0);
            }
        }
    }
    println!("{:?}", intersections);
}

fn parse_path(path: &String) -> Vec<(i32, i32)> {
    let steps: Vec<String> = path.split(',').map(|s| s.parse().unwrap()).collect();
    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push((0, 0));
    for step in steps {
        let current_coord = coords.last().unwrap();
        coords.push(get_new_coord(current_coord, step));
    }
    println!("{:?}", coords);
    coords
}

fn get_new_coord(current_coord: &(i32, i32), step: String) -> (i32, i32) {
    let (x, y) = current_coord;
    let direction = &step[..1];
    let distance: i32 = (&step[1..]).parse().unwrap();
    match direction {
        "R" => (*x + distance, *y),
        "L" => (*x - distance, *y),
        "U" => (*x, *y + distance),
        "D" => (*x, *y - distance),
        &_ => todo!(),
    }
}

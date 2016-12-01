use std::collections::HashSet;

fn main() {
    #[derive(Debug)]
    enum Directions {
        Nord,
        Sued,
        Ost,
        West,
    }

    let input_str =
        "R4, R1, L2, R1, L1, L1, R1, L5, R1, R5, L2, R3, L3, L4, R4, R4, R3, L5, L1, R5, R3, L4, \
         R1, R5, L1, R3, L2, R3, R1, L4, L1, R1, L1, L5, R1, L2, R2, L3, L5, R1, R5, L1, R188, \
         L3, R2, R52, R5, L3, R79, L1, R5, R186, R2, R1, L3, L5, L2, R2, R4, R5, R5, L5, L4, R5, \
         R3, L4, R4, L4, L4, R5, L4, L3, L1, L4, R1, R2, L5, R3, L4, R3, L3, L5, R1, R1, L3, R2, \
         R1, R2, R2, L4, R5, R1, R3, R2, L2, L2, L1, R2, L1, L3, R5, R1, R4, R5, R2, R2, R4, R4, \
         R1, L3, R4, L2, R2, R1, R3, L5, R5, R2, R5, L1, R2, R4, L1, R5, L3, L3, R1, L4, R2, L2, \
         R1, L1, R4, R3, L2, L3, R3, L2, R1, L4, R5, L1, R5, L2, L1, L5, L2, L5, L2, L4, L2, R3";

    // let input_str = "R8, R4, R4, R8";

    let mut end_x: isize = 0;
    let mut end_y: isize = 0;

    let mut dir = Directions::Nord;

    let mut pos = Vec::<(isize, isize)>::new();

    for d in input_str.split(", ") {
        match &d[..1] {
            "R" => {
                match dir {
                    Directions::Nord => {
                        end_x += d[1..].parse().unwrap();
                        dir = Directions::Ost;
                    }
                    Directions::Ost => {
                        end_y -= d[1..].parse().unwrap();
                        dir = Directions::Sued;
                    }
                    Directions::Sued => {
                        end_x -= d[1..].parse().unwrap();
                        dir = Directions::West;
                    }
                    Directions::West => {
                        end_y += d[1..].parse().unwrap();
                        dir = Directions::Nord;
                    }
                }
            }
            "L" => {
                match dir {
                    Directions::Nord => {
                        end_x -= d[1..].parse().unwrap();
                        dir = Directions::West;
                    }
                    Directions::Ost => {
                        end_y += d[1..].parse().unwrap();
                        dir = Directions::Nord;
                    }
                    Directions::Sued => {
                        end_x += d[1..].parse().unwrap();
                        dir = Directions::Ost;
                    }
                    Directions::West => {
                        end_y -= d[1..].parse().unwrap();
                        dir = Directions::Sued;
                    }
                }
            }
            _ => (),
        }
        pos.push((end_x, end_y));
    }
    println!("Part1: {}:{} ==> {}",
             end_x,
             end_y,
             end_x.abs() + end_y.abs());
    println!("{:?}", pos);

    let mut start_pos = (0, 0);
    let mut h_set = HashSet::new();
    h_set.insert(start_pos);
    'outer: for p in pos {
        let x_delta = p.0 - start_pos.0;
        let y_delta = p.1 - start_pos.1;
        if x_delta < 0 {
            for _ in 0..x_delta.abs() {
                start_pos.0 -= 1;
                if !h_set.contains(&start_pos) {
                    h_set.insert(start_pos);
                } else {
                    println!("Part 2: {}:{} ==> {}",
                             start_pos.0,
                             start_pos.1,
                             start_pos.0.abs() + start_pos.1.abs());
                    break 'outer;
                }
            }
        } else if x_delta > 0 {
            for _ in 0..x_delta.abs() {
                start_pos.0 += 1;
                if !h_set.contains(&start_pos) {
                    h_set.insert(start_pos);
                } else {
                    println!("Part 2: {}:{} ==> {}",
                             start_pos.0,
                             start_pos.1,
                             start_pos.0.abs() + start_pos.1.abs());
                    break 'outer;
                }
            }
        } else if y_delta < 0 {
            for _ in 0..y_delta.abs() {
                start_pos.1 -= 1;
                if !h_set.contains(&start_pos) {
                    h_set.insert(start_pos);
                } else {
                    println!("Part 2: {}:{} ==> {}",
                             start_pos.0,
                             start_pos.1,
                             start_pos.0.abs() + start_pos.1.abs());
                    break 'outer;
                }
            }
        } else if y_delta > 0 {
            for _ in 0..y_delta.abs() {
                start_pos.1 += 1;
                if !h_set.contains(&start_pos) {
                    h_set.insert(start_pos);
                } else {
                    println!("Part 2: {}:{} ==> {}",
                             start_pos.0,
                             start_pos.1,
                             start_pos.0.abs() + start_pos.1.abs());
                    break 'outer;
                }
            }
        }
        start_pos = p;
    }
}

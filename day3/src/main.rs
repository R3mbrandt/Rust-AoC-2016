use std::fs::File;
use std::io::Read;

fn get_data(fname: &str) -> String {
    let mut file = match File::open(fname) {
        Err(e) => panic!("file error: {}", e),
        Ok(file) => file,
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Err(e) => panic!("read error: {}", e),
        Ok(_) => {}
    }
    data
}

fn main() {
    let mut valid_ones = Vec::<Vec<usize>>::new();
    let mut data = Vec::<Vec<usize>>::new();

    // --- Part 1
    for l in get_data("src/input.txt").lines() {
        let mut d: Vec<usize> = l.split_whitespace().map(|e| e.trim().parse().unwrap()).collect();
        data.push(d.clone());  // für Part 2
        d.sort();
        if d[0] + d[1] > d[2] {
            valid_ones.push(d);
        }
    }
    println!("Part 1: {}", valid_ones.len());
    // --- Part 2
    valid_ones.clear();
    let mut one_column = Vec::new();
    for i in 0..3 {
        for d in &data {
            one_column.push(d[i])
        }
    }
    for c in one_column.chunks(3) {
        let mut d = vec![c[0], c[1], c[2]];
        d.sort();
        if d[0] + d[1] > d[2] {
            valid_ones.push(d);
        }
    }
    println!("Part 2: {}", valid_ones.len());
}
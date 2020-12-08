use std::fs;

const FILE_NAME: &str = "dataset";

fn main () {
    let dataset: String = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong!");
    let mut total: u32 = 0;
    let mut ribbon: u32 = 0;
    for line in dataset.lines() {
        let mut dim = line.split('x').collect::<Vec<&str>>();
        let size: u16 =
              dim[0].parse::<u16>().unwrap() * dim[1].parse::<u16>().unwrap() * 2
            + dim[1].parse::<u16>().unwrap() * dim[2].parse::<u16>().unwrap() * 2
            + dim[0].parse::<u16>().unwrap() * dim[2].parse::<u16>().unwrap() * 2
            + get_minimum(
                dim[0].parse::<u16>().unwrap() * dim[1].parse::<u16>().unwrap(),
                dim[1].parse::<u16>().unwrap() * dim[2].parse::<u16>().unwrap(),
                dim[0].parse::<u16>().unwrap() * dim[2].parse::<u16>().unwrap()
            );
        ribbon += get_shorter_two(&mut dim) as u32
            + dim[0].parse::<u32>().unwrap()
            * dim[1].parse::<u32>().unwrap()
            * dim[2].parse::<u32>().unwrap();
        total += size as u32;
    }
    println!("{}\n{}", total, ribbon);
}

fn get_minimum (a: u16, b: u16, c: u16) -> u16 {
    let mut min: u16 = a;
    if b < min {
        min = b;
    }
    if c < min {
        min = c;
    }
    min
}

fn get_shorter_two (vector: &mut Vec<&str>) -> u8 {
    sort(vector);
    vector[0].parse::<u8>().unwrap() * 2 + vector[1].parse::<u8>().unwrap() * 2
}

fn sort(vector: &mut Vec<&str>) {
    for i in 0..vector.len() {
        for j in 0..vector.len() - i - 1 {
            if vector[j + 1].parse::<u8>().unwrap() < vector[j].parse::<u8>().unwrap() {
                vector.swap(j, j + 1);
            }
        }
    }
}
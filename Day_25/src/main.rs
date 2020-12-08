fn main () {
    let mut point: (u16, u16, u32) = (1, 1, 20151125);
    loop {
        if point.0 == 2947 && point.1 == 3029 {
            println!("{}", point.2);
            break;
        }
        point.0 -= 1;
        point.1 += 1;
        if point.0 < 1 {
            point.0 = point.1;
            point.1 = 1;
        }
        point.2 = compute_next(point.2);
    }
}

fn compute_next(a: u32) -> u32 {
    let result = a as u64 * 252533 % 33554393;
    result as u32
}
const DIVIDER: usize = 20201227;

const PUB_KEY_ONE: usize = 3418282;
const PUB_KEY_TWO: usize = 8719412;

fn main() {
    let loop_size_1 = find_loop_size(PUB_KEY_ONE, 7);
    let loop_size_2 = find_loop_size(PUB_KEY_TWO, 7);
    println!("Loop size 1: {}", loop_size_1);
    println!("Loop size 2: {}", loop_size_2);

    // Should be the same :
    let encryption_key_1 = transform(PUB_KEY_TWO, loop_size_1);
    let encryption_key_2 = transform(PUB_KEY_ONE, loop_size_2);
    assert_eq!(encryption_key_1, encryption_key_2);

    println!("Part 1 result: {}", encryption_key_1);


}

fn find_loop_size(target: usize, subject: usize) -> usize {
    let mut loop_count = 0;
    let mut value = 1;
    while value != target {
        loop_count += 1;
        value = transform_step(value, subject);
    }
    loop_count
}

fn transform(subject: usize, loops: usize) -> usize {
    let mut value = 1;
    for _ in 0..loops {
        value = transform_step(value, subject);
    }
    value
}

fn transform_step(value: usize, subject: usize) -> usize {
    (value * subject) % DIVIDER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

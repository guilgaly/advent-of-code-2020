use common::time_execution;

static INPUT: [usize; 6] = [7, 12, 1, 0, 16, 2];

fn main() -> Result<(), String> {
    let res_1 = time_execution("Part 1", || search(&INPUT, 2020));
    println!("Part 1 result: {}", res_1);

    let res_2 =  time_execution("Part 2", || search(&INPUT, 30000000));
    println!("Part 2 result: {}", res_2);

    Ok(())
}

fn search(input: &[usize], limit: usize) -> usize {
    let mut cache: Vec<usize> = vec![0; limit];

    for (idx, value) in input.iter().copied().enumerate() {
        cache[value] = idx + 1;
    }

    let mut previous_number = 0;
    let start_turn = input.len() + 1;
    for turn in start_turn..limit {
        let next = match cache[previous_number] {
            0 => 0,
            previous_turn => turn - previous_turn,
        };
        cache[previous_number] = turn;
        previous_number = next;
    }

    previous_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search(&[0, 3, 6], 2020), 436)
    }
}

use std::collections::HashSet;

use common::itertools::Itertools;
use common::time_execution;

const INPUT: [usize; 9] = [4, 6, 9, 2, 1, 7, 5, 3, 8];

/// - value at pointers[i] is the label of the cup located just after (clockwise of) the cup labelled i
/// - pointers[0] is never used since labels start from 1 (saves having to shift indexes by -1)
type Pointers = [usize];

fn main() -> Result<(), String> {
    let res_1 = time_execution("Part 1", || play_game(&INPUT, 9, 100));
    println!("Part 1 result: {}", res_1.iter().join(""));

    let res_2 = time_execution("Part 2", || play_game(&INPUT, 1_000_000, 10_000_000));
    println!("Part 2 result: {}", res_2[0] * res_2[1]);

    Ok(())
}

fn play_game(init: &Pointers, max_cup: usize, moves: usize) -> Vec<usize> {
    let mut pointers: Vec<usize> = (1..=(max_cup + 1)).collect();
    for w in init.windows(2) {
        pointers[w[0]] = w[1];
    }

    if init.len() == max_cup {
        // wrap last pointer to the beginning
        pointers[init[init.len() - 1]] = init[0];
    } else {
        // connect the last pointer from the init values to the first generated value;
        // then wrap the last pointer from the generated values to the beginning
        pointers[init[init.len() - 1]] = init.len() + 1;
        pointers[max_cup] = init[0];
    }

    let mut current = init[0];
    for _ in 0..moves {
        current = play_next_move(&mut pointers, current, max_cup);
    }

    follow_pointers_after(&pointers, 1)
}

fn follow_pointers_after(pointers: &Pointers, current: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut p = current;
    loop {
        p = pointers[p];
        if p == current {
            // We've come full circle!
            return res;
        }
        res.push(p);
    }
}

fn play_next_move(pointers: &mut Pointers, current: usize, max_cup: usize) -> usize {
    // take out three
    let cup_0 = pointers[current]; // first cup taken out
    let cup_1 = pointers[cup_0]; // second cup taken out
    let cup_2 = pointers[cup_1]; // third cup taken out
    let cups_taken = [cup_0, cup_1, cup_2];

    pointers[current] = pointers[cup_2];

    // destination: reduce value by 1 until valid
    let decrement_dest = |current: usize| if current > 1 { current - 1 } else { max_cup };
    let mut destination = decrement_dest(current);
    while cups_taken.contains(&destination) {
        destination = decrement_dest(destination);
    }

    // insert cups that were taken out
    pointers[cup_2] = pointers[destination];
    pointers[destination] = cup_0;

    // return the new current cup
    pointers[current]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

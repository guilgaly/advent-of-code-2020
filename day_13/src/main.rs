use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    part_1()?;

    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), String> {
    let mut lines = INPUT.lines();

    let t0 = lines
        .next()
        .and_then(|line| line.parse::<usize>().ok())
        .ok_or("No valid t0")?;
    println!("t0: {}", t0);

    let ids: Vec<usize> = lines
        .next()
        .iter()
        .flat_map(|line| line.split(',').filter_map(|v| v.parse::<usize>().ok()))
        .collect();

    let res_1 = ids
        .iter()
        .map(|id| (*id, find_next_departure(*id, t0)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .ok_or("Part 1 result not found")?;
    println!("Part 1 result: {}", res_1.0 * (res_1.1 - t0));

    Ok(())
}

fn part_2() -> Result<(), String> {
    let mut lines = INPUT.lines();

    let t0 = lines
        .next()
        .and_then(|line| line.parse::<i64>().ok())
        .ok_or("No valid t0")?;
    println!("t0: {}", t0);

    // (Index, Line ID)
    let busses: Vec<(usize, i64)> = lines
        .next()
        .iter()
        .flat_map(|line| {
            line.split(',')
                .enumerate()
                .filter_map(|(i, v)| v.parse::<i64>().ok().map(|id| (i, id)))
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect();
    println!("busses: {:?}", busses);

    let N = busses.iter().map(|(_, id)| id).product::<i64>();
    println!("N: {}", N);
    let res = busses
        .iter()
        .map(|(i, id)| {
            let Ni = N / id;
            let bi = (id - *i as i64) % id;
            println!("Ni: {}, bi: {}", Ni, bi);
            if bi == 0 {
                0
            } else {
                let mut x = 0;
                while (Ni * x) % id != 1 {
                    x += 1;
                }
                bi * Ni * x
            }
        })
        .sum::<i64>();

    println!("Part 2 result: {}", res % N);

    Ok(())
}

fn find_next_departure(id: usize, after: usize) -> usize {
    let mut next = 0;
    while next < after {
        next += id;
    }
    next
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}

use common::lazy_static::lazy_static;
use common::regex::Regex;
use std::str::FromStr;
use common::itertools::Itertools;

static INPUT_MY_TICKET: &str = include_str!("my_ticket");
static INPUT_OTHER_TICKETS: &str = include_str!("other_tickets");
static INPUT_TICKET_RULES: &str = include_str!("ticket_rules");

fn main() -> Result<(), String> {
    let my_ticket = parse_ticket(INPUT_MY_TICKET)?;
    let other_tickets = INPUT_OTHER_TICKETS
        .lines()
        .map(|line| parse_ticket(line))
        .collect::<Result<Vec<Ticket>, String>>()?;
    let rules: Vec<TicketRule> = INPUT_TICKET_RULES
        .lines()
        .map(|line| line.parse::<TicketRule>())
        .collect::<Result<Vec<TicketRule>, String>>()?;
    println!("rules: {}", rules.len());

    let mut scan_error_rate: usize = 0;
    let valid_tickets: Vec<&Ticket> = other_tickets
        .iter()
        .filter(|ticket| {
            let scan_errors: Vec<usize> = ticket
                .iter()
                .filter(|field| !rules.iter().any(|rule| rule.is_valid(**field)))
                .copied()
                .collect();
            scan_error_rate += scan_errors.iter().sum::<usize>();
            scan_errors.is_empty()
        })
        .collect();
    println!("Part 1 result: {}", scan_error_rate);
    println!("Part 1 valid_tickets: {}", valid_tickets.len());

    // Find all known valid values for each field
    let mut values_by_field: [Vec<usize>; 20] = Default::default();
    for (i, values) in values_by_field.iter_mut().enumerate() {
        for ticket in &valid_tickets {
            values.push(ticket[i]);
        }
    }

    let mut rules_with_possible_idxes: Vec<(&TicketRule, Vec<usize>)> = rules
        .iter()
        .map(|rule| {
            let possible_idxes: Vec<usize> = ((0 as usize)..20)
                .filter(|i| values_by_field[*i].iter().all(|v| rule.is_valid(*v)))
                .collect();
            (rule, possible_idxes)
        })
        .sorted_by_key(|(_, idxes)| idxes.len())
        .collect();
    let length = rules_with_possible_idxes.len();
    for x in 0..length {
        let idx = *rules_with_possible_idxes[x].1.first().unwrap();
        for y in (x + 1)..length {
            let idx_to_remove = rules_with_possible_idxes[y]
                .1
                .iter()
                .position(|v| *v == idx)
                .unwrap();
            rules_with_possible_idxes[y].1.remove(idx_to_remove);
        }
    }

    let res_2 = rules_with_possible_idxes
        .iter()
        .filter_map(|(rule, idxes)| {
            if rule.name.starts_with("departure") {
                Some(my_ticket[*idxes.first().unwrap()])
            } else {
                None
            }
        })
        .product::<usize>();
    println!("Part 2 result: {}", res_2);

    Ok(())
}

type Ticket = Box<[usize; 20]>;

fn parse_ticket(s: &str) -> Result<Ticket, String> {
    let vec = s
        .split(',')
        .map(|x| {
            x.parse::<usize>()
                .map_err(|_| format!("Not a valid ticket field: {}", x))
        })
        .collect::<Result<Vec<usize>, String>>()?;
    if vec.len() != 20 {
        Err("Not a valid ticket".to_owned())
    } else {
        let mut ticket: Ticket = Box::new([0; 20]);
        vec.iter()
            .enumerate()
            .for_each(|(i, field)| ticket[i] = *field);
        Ok(ticket)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct TicketRule {
    name: String,
    min_1: usize,
    max_1: usize,
    min_2: usize,
    max_2: usize,
}

impl TicketRule {
    fn is_valid(&self, value: usize) -> bool {
        (value >= self.min_1 && value <= self.max_1) || (value >= self.min_2 && value <= self.max_2)
    }
}

impl FromStr for TicketRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
        }
        REGEX
            .captures(s)
            .and_then(|cap| {
                let name = cap.get(1)?.as_str().to_owned();
                let min_1 = cap.get(2)?.as_str().parse::<usize>().ok()?;
                let max_1 = cap.get(3)?.as_str().parse::<usize>().ok()?;
                let min_2 = cap.get(4)?.as_str().parse::<usize>().ok()?;
                let max_2 = cap.get(5)?.as_str().parse::<usize>().ok()?;
                Some(TicketRule {
                    name,
                    min_1,
                    max_1,
                    min_2,
                    max_2,
                })
            })
            .ok_or(format!("Cannot parse as TicketRule: {}", s))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}

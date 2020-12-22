use std::collections::HashMap;

static INPUT_1: &str = include_str!("input1");
static INPUT_1_FIXED: &str = include_str!("input1_fixed");
static INPUT_2: &str = include_str!("input2");

fn main() -> Result<(), String> {
    let rules_by_id = INPUT_1.lines().map(|line| rule_with_id(line)).collect::<HashMap<u64, Rule>>();
    println!("Part 1 result: {}", solve(&rules_by_id));

    let rules_by_id_fixed = INPUT_1_FIXED.lines().map(|line| rule_with_id(line)).collect::<HashMap<u64, Rule>>();
    println!("Part 2 result: {}", solve(&rules_by_id_fixed));

    Ok(())
}

fn solve(rules_by_id: &HashMap<u64, Rule>) -> i32 {
    let mut c = 0;
    for msg in INPUT_2.lines() {
        let msg: Vec<_> = msg.chars().collect();
        for m in rules_by_id.get(&0).unwrap().matches(&rules_by_id, &msg).into_iter() {
            if m.is_empty() {
                c += 1;
                break;
            }
        }
    }
    c
}

fn rule_with_id(line: &str) -> (u64, Rule) {
    let mut split = line.split(": ");
    let rule_id = split.next().unwrap().parse::<u64>().unwrap();
    let rule_body = parse_rule(split.next().unwrap());
    (rule_id, rule_body)
}

fn parse_rule(s: &str) -> Rule {
    if s.contains(" | ") {
        let parts: Vec<&str> = s.split(" | ").collect();
        Rule::Or(Box::new(parse_rule(parts[0])), Box::new(parse_rule(parts[1])))
    } else if s.starts_with('"') {
        Rule::Char(s.chars().nth(1).unwrap())
    } else if s.contains(' ') {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() == 2 {
            Rule::Seq2(Box::new(parse_rule(parts[0])), Box::new(parse_rule(parts[1])))
        } else if parts.len() == 3 {
            Rule::Seq3(Box::new(parse_rule(parts[0])), Box::new(parse_rule(parts[1])), Box::new(parse_rule(parts[2])))
        } else {
            panic!("Cannot parse '{}' (Seq of length {})", s,parts.len());
        }

    } else if let Ok(i) = s.parse::<u64>() {
        Rule::Ref(i)
    } else {
        panic!("Cannot parse '{}'", s);
    }
}

enum Rule {
    Ref(u64),
    Char(char),
    Seq2(Box<Rule>, Box<Rule>),
    Seq3(Box<Rule>, Box<Rule>, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
}

impl Rule {
    fn matches<'a>(&self, rules_by_id: &'a HashMap<u64, Rule>, unparsed: &'a [char]) -> Vec<&'a [char]> {
        if unparsed.is_empty() {
            vec![]
        } else {
            match self {
                Rule::Ref(i) => rules_by_id.get(i).unwrap().matches(rules_by_id, unparsed),
                Rule::Char(c) => if unparsed[0] == *c { vec![&unparsed[1..]] } else { vec![] },
                Rule::Seq2(a, b) => {
                    let mut r = Vec::new();
                    for m in a.matches(rules_by_id, unparsed).into_iter() {
                        for n in b.matches(rules_by_id, m) {
                            r.push(n);
                        }
                    }
                    r
                }
                Rule::Seq3(a, b, c) => {
                    let mut r = Vec::new();
                    for m in a.matches(rules_by_id, unparsed).into_iter() {
                        for n in b.matches(rules_by_id, m) {
                            for o in c.matches(rules_by_id, n) {
                                r.push(o);
                            }
                        }
                    }
                    r
                }
                Rule::Or(a, b) => {
                    let mut r = Vec::new();
                    for a in a.matches(rules_by_id, unparsed).into_iter() {
                        r.push(a);
                    }
                    for b in b.matches(rules_by_id, unparsed).into_iter() {
                        r.push(b);
                    }
                    r
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

use std::collections::{HashSet, VecDeque};

static INPUT_1: &str = include_str!("input_player_1");
static INPUT_2: &str = include_str!("input_player_2");

fn main() -> Result<(), String> {
    let deck_1 = parse_deck(INPUT_1)?;
    let deck_2 = parse_deck(INPUT_2)?;

    println!("Part 1 result: {}", play_combat(deck_1.clone(), deck_2.clone()));

    println!(
        "Part 2 result: {}",
        play_recursive_combat(deck_1.clone(), deck_2.clone()).1
    );

    Ok(())
}

fn play_combat(deck_1: VecDeque<u64>, deck_2: VecDeque<u64>) -> u64 {
    let mut deck_1 = deck_1;
    let mut deck_2 = deck_2;

    while !deck_1.is_empty() && !deck_2.is_empty() {
        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        if card_1 > card_2 {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        } else {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
    }

    let winning_deck = if deck_1.is_empty() { deck_2 } else { deck_1 };
    count_score(&winning_deck)
}

fn play_recursive_combat(deck_1: VecDeque<u64>, deck_2: VecDeque<u64>) -> (Winner, u64) {
    let mut deck_1 = deck_1;
    let mut deck_2 = deck_2;

    let mut previous_decks: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();

    loop {
        let state = (deck_1.clone(), deck_2.clone());
        if previous_decks.contains(&state) {
            return (Winner::Player1, count_score(&deck_1));
        }
        previous_decks.insert(state);

        let card_1 = deck_1.pop_front().unwrap();
        let card_2 = deck_2.pop_front().unwrap();

        let winner = if deck_1.len() as u64 >= card_1 && deck_2.len() as u64 >= card_2 {
            play_recursive_combat(copy_top_n_cards(&deck_1, card_1), copy_top_n_cards(&deck_2, card_2)).0
        } else {
            if card_1 > card_2 {
                Winner::Player1
            } else {
                Winner::Player2
            }
        };

        match winner {
            Winner::Player1 => {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
            }
            Winner::Player2 => {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
            }
        }

        if deck_1.is_empty() {
            return (Winner::Player2, count_score(&deck_2));
        } else if deck_2.is_empty() {
            return (Winner::Player1, count_score(&deck_1));
        }
    }
}

fn count_score(winning_deck: &VecDeque<u64>) -> u64 {
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i as u64 + 1) * *card)
}

fn copy_top_n_cards(deck: &VecDeque<u64>, n: u64) -> VecDeque<u64> {
    deck.iter().take(n as usize).copied().collect()
}

/// Top card is at the beginning, bottom card at the end
fn parse_deck(input: &str) -> Result<VecDeque<u64>, String> {
    input
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|e| format!("Cannot parse {}: {}", line, e))
        })
        .collect()
}

enum Winner {
    Player1,
    Player2,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck(cards: &[u64]) -> VecDeque<u64> {
        cards.iter().copied().collect::<VecDeque<_>>()
    }

    fn deck_1() -> VecDeque<u64> {
        deck(&[9, 2, 6, 3, 1])
    }

    fn deck_2() -> VecDeque<u64> {
        deck(&[5, 8, 4, 7, 10])
    }

    #[test]
    fn test_combat() {
        assert_eq!(play_combat(deck_1(), deck_2()), 306);
    }

    #[test]
    fn test_recursive_combat() {
        assert_eq!(play_recursive_combat(deck_1(), deck_2()).1, 291);
    }
}

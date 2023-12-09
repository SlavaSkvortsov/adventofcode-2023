use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::char::from_digit;
use std::cmp::Ordering;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn card_to_number(card: char) -> u8 {
    match card {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

struct Hand {
    cards: Vec<u8>,
    bid: u64,
}

fn count_element_function<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}




fn compare(a: &Hand, b: &Hand) -> Ordering {
    if a.cards == b.cards {
        return Ordering::Equal;
    }

    let mut a_count = count_element_function(a.cards.iter());
    let a_joker_count = a_count.remove(&1).unwrap_or(0);

    let mut b_count = count_element_function(b.cards.iter());
    let b_joker_count = b_count.remove(&1).unwrap_or(0);

    let mut a_count_max = *a_count.values().max().unwrap_or(&0);
    let mut b_count_max = *b_count.values().max().unwrap_or(&0);

    for (card, count) in a_count.iter_mut() {
        if *count == a_count_max {
            *count += a_joker_count;
            break
        }
    }

    for (card, count) in b_count.iter_mut() {
        if *count == b_count_max {
            *count += b_joker_count;
            break
        }
    }

    a_count_max += a_joker_count;
    b_count_max += b_joker_count;

    // One is definitely bigger than the other
    if a_count_max > b_count_max {
        return Ordering::Greater;
    } else if a_count_max < b_count_max {
        return Ordering::Less;
    }

    // max count is the same, check for full house and two pairs
    // Full house example:

    // a = [2, 2, 2, 3, 3]
    // a_count = {2: 3, 3: 2}
    // a_count_count = {3: 1, 2: 1}

    // b = [1, 2, 3, 3, 3]
    // b_count = {3: 3, 1: 1, 2: 1}
    // b_count_count = {3: 1, 1: 2}

    // Two pairs example:
    // a = [1, 1, 2, 2, 3]
    // a_count = {1: 2, 2: 2, 3: 1}
    // a_count_count = {2: 2, 1: 1}

    // b = [1, 1, 2, 3, 4]
    // b_count = {1: 2, 2: 1, 3: 1, 4: 1}
    // b_count_count = {2: 1, 1: 3}
    let a_count_count = count_element_function(a_count.values());
    let b_count_count = count_element_function(b_count.values());

    for i in vec![3, 2, 1].iter() {
        let a_count_count_value = *a_count_count.get(&i).unwrap_or(&0);
        let b_count_count_value = *b_count_count.get(&i).unwrap_or(&0);
        if a_count_count_value > b_count_count_value {
            return Ordering::Greater;
        } else if a_count_count_value < b_count_count_value {
            return Ordering::Less;
        }
    }

    // Both are the same, check for highest card while preserving order
    for i in 0..5 {
        if a.cards[i] > b.cards[i] {
            return Ordering::Greater;
        } else if a.cards[i] < b.cards[i] {
            return Ordering::Less;
        }
    }

    // Both are the same, return false
    return Ordering::Equal;
}

fn get_combo_strength(cards: &Vec<u8>) -> u8 {
    let mut count = count_element_function(cards.iter());
    let joker_count = count.remove(&1).unwrap_or(0);

    let count_max = *count.values().max().unwrap_or(&0);

    // Five of a kind
    if count_max + joker_count == 5 {
        return 10;
    }

    // Four of a kind
    if count_max + joker_count == 4 {
        return 9;
    }

    // Full house
    if count_max == 3 {
        if joker_count == 0 && count.len() == 2 {
            return 8;
        }
    }

    let mut count_sorted: Vec<usize> = count.values().map(|x| *x).collect::<Vec<usize>>();
    count_sorted.sort();
    count_sorted.reverse();
    // Full house with joker
    if count_sorted[0] == 2 && count_sorted[1] == 2 && joker_count == 1 {
        return 8;
    }

    // Three of a kind
    if count_max + joker_count == 3 {
        return 5;
    }

    // Two pairs
    if count_max == 2 && count.len() == 3 {
        return 4;
    }

    // Pair
    if count_max + joker_count == 2 {
        return 3;
    }

    // High card
    return 2;
}


fn lame_compare(a: &Hand, b: &Hand) -> Ordering {
    if get_combo_strength(&a.cards) > get_combo_strength(&b.cards) {
        return Ordering::Greater;
    } else if get_combo_strength(&a.cards) < get_combo_strength(&b.cards) {
        return Ordering::Less;
    }

    // Both are the same, check for highest card while preserving order
    for i in 0..5 {
        if a.cards[i] > b.cards[i] {
            return Ordering::Greater;
        } else if a.cards[i] < b.cards[i] {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}


fn double_compare(a: &Hand, b: &Hand) -> Ordering {
    let lame_compare_result = lame_compare(a, b);
    let compare_result = compare(a, b);
    if lame_compare_result != compare_result {
        print!("A hand = {:?}, B hand = {:?}\n", a.cards, b.cards);
        panic!("Lame compare result: {:?}, compare result: {:?}", lame_compare_result, compare_result);
    }

    return compare_result;
}


fn part_1() {
    let mut hands: Vec<Hand> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let (cards_str, bid) = row.split_once(" ").unwrap();
                let cards: Vec<u8> = cards_str.chars().map(|c| card_to_number(c)).collect();

                hands.push(Hand { cards, bid: bid.parse::<u64>().unwrap() });
            }
        }
    }

    hands.sort_by(|a, b| double_compare(a, b));

    let mut result: u64 = 0;

    for (i, hand) in hands.iter().enumerate() {
        println!("Hand: {:?}, bid: {}, multiplier: {}", hand.cards, hand.bid, i + 1);
        result += hand.bid * (i as u64 + 1);
        println!("result: {}", result);
    }

    println!("Result: {}", result);
}


fn test_compare() {
    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![3, 3, 3, 3, 3], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Five of a kind vs Four of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![3, 3, 3, 3, 4], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Full house
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![3, 3, 3, 4, 4], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Full house vs Full house
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![3, 3, 3, 4, 4], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Full house
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![2, 2, 3, 3, 3], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Full house
    let a = Hand { cards: vec![3, 3, 2, 2, 3], bid: 1 };
    let b = Hand { cards: vec![3, 3, 2, 3, 2], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![1, 1, 1, 5, 5], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Four of a kind
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![1, 1, 5, 5, 6], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Four of a kind
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![1, 1, 1, 5, 6], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // A hand = [2, 2, 3, 3, 3], B hand = [13, 4, 3, 3, 3]
    // Full house vs Three of a kind
    let a = Hand { cards: vec![2, 2, 3, 3, 3], bid: 1 };
    let b = Hand { cards: vec![13, 4, 3, 3, 3], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // High card vs High card
    let a = Hand { cards: vec![2, 3, 4, 5, 6], bid: 1 };
    let b = Hand { cards: vec![3, 4, 5, 6, 7], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // High card vs High card
    let a = Hand { cards: vec![2, 3, 4, 5, 6], bid: 1 };
    let b = Hand { cards: vec![2, 3, 4, 5, 7], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Two pairs vs Three of a kind
    let a = Hand { cards: vec![2, 2, 3, 3, 4], bid: 1 };
    let b = Hand { cards: vec![1, 1, 5, 6, 7], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Pair vs Three of a kind
    let a = Hand { cards: vec![2, 2, 3, 4, 5], bid: 1 };
    let b = Hand { cards: vec![1, 1, 3, 4, 5], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Pair vs Pair - kicker
    let a = Hand { cards: vec![2, 2, 3, 4, 5], bid: 1 };
    let b = Hand { cards: vec![2, 2, 3, 4, 6], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Four of a kind vs Four of a kind - kicker
    let a = Hand { cards: vec![2, 2, 2, 2, 3], bid: 1 };
    let b = Hand { cards: vec![2, 2, 2, 2, 4], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Four of a kind vs Four of a kind - kicker
    let a = Hand { cards: vec![2, 2, 2, 2, 3], bid: 1 };
    let b = Hand { cards: vec![2, 2, 2, 2, 4], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Three of a kind vs Three of a kind - kicker
    let a = Hand { cards: vec![2, 2, 2, 3, 4], bid: 1 };
    let b = Hand { cards: vec![2, 2, 2, 3, 5], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Full house vs Four of a kind
    let a = Hand { cards: vec![2, 2, 2, 3, 3], bid: 1 };
    let b = Hand { cards: vec![1, 1, 5, 5, 6], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 1, 1, 1, 1], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 2, 1, 1, 1], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 2, 2, 1, 1], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 2, 2, 2, 1], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 2, 2, 2, 2], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 1, 2, 2, 2], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 1, 1, 2, 2], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Five of a kind vs Five of a kind
    let a = Hand { cards: vec![2, 2, 2, 2, 2], bid: 1 };
    let b = Hand { cards: vec![1, 1, 1, 1, 2], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);

    // Two pairs vs Thee of a kind
    let a = Hand { cards: vec![2, 2, 3, 3, 4], bid: 1 };
    let b = Hand { cards: vec![1, 3, 2, 2, 5], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // Four of a kind vs Four of a kind with joker
    let a = Hand { cards: vec![2, 2, 2, 2, 3], bid: 1 };
    let b = Hand { cards: vec![2, 2, 2, 3, 1], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);

    // High card vs Pair with Joker
    // A hand = [12, 13, 10, 3, 14], B hand = [4, 1, 9, 3, 5]
    let a = Hand { cards: vec![12, 13, 10, 3, 14], bid: 1 };
    let b = Hand { cards: vec![4, 1, 9, 3, 5], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Less);
    assert_eq!(compare(&b, &a), Ordering::Greater);
    assert_eq!(lame_compare(&a, &b), Ordering::Less);
    assert_eq!(lame_compare(&b, &a), Ordering::Greater);

    // All jokers vs four of a kind
    let a = Hand { cards: vec![1, 1, 1, 1, 1], bid: 1 };
    let b = Hand { cards: vec![13, 13, 13, 3, 13], bid: 1 };
    assert_eq!(compare(&a, &b), Ordering::Greater);
    assert_eq!(compare(&b, &a), Ordering::Less);


    println!("Tests are over")
}



fn part_2() {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {

            }
        }
    }
}

fn main() {
    // test_compare();
    part_1();
    // part_2();
}
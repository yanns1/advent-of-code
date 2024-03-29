use core::panic;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    iter::zip,
};

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(?<hand>\w{5}) (?<bid>\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("'{c}' is not a valid card."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand(pub Vec<Card>);

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        assert!(
            cards.len() == 5,
            "A hand is composed of exactly 5 cards, but got {}.",
            cards.len()
        );
        Self(cards)
    }

    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, u16> = HashMap::new();
        for card in self.0.iter() {
            let card = card.clone();
            counts
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut counts: Vec<u16> = counts.into_values().collect();
        counts.sort_unstable();

        if counts.len() == 1 && counts[0] == 5 {
            HandType::FiveOfAKind
        } else if counts.len() == 2 && counts[0] == 1 && counts[1] == 4 {
            HandType::FourOfAKind
        } else if counts.len() == 2 && counts[0] == 2 && counts[1] == 3 {
            HandType::FullHouse
        } else if counts.len() == 3 && counts[0] == 1 && counts[1] == 1 && counts[2] == 3 {
            HandType::ThreeOfAKind
        } else if counts.len() == 3 && counts[0] == 1 && counts[1] == 2 && counts[2] == 2 {
            HandType::TwoPair
        } else if counts.len() == 4
            && counts[0] == 1
            && counts[1] == 1
            && counts[2] == 1
            && counts[3] == 2
        {
            HandType::OnePair
        } else if counts.len() == 5 && counts.iter().all(|&count| count == 1) {
            HandType::HighCard
        } else {
            panic!("Got unexpected `counts`. There must be a bug in this function.");
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                for (c1, c2) in zip(&self.0, &other.0) {
                    match c1.cmp(c2) {
                        std::cmp::Ordering::Equal => {
                            continue;
                        }
                        ordering => {
                            return ordering;
                        }
                    }
                }
                std::cmp::Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Bid = u64;

#[derive(Debug)]
struct HandBid {
    hand: Hand,
    bid: Bid,
}

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut handbids: Vec<HandBid> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let cap = LINE_RE.captures(&line).unwrap();
            let hand: Vec<Card> = cap["hand"].chars().map(Card::new).collect();
            let hand = Hand::new(hand);
            let bid: u64 = cap["bid"].parse().unwrap();
            HandBid { hand, bid }
        })
        .collect();

    handbids.sort_unstable_by_key(|handbid| handbid.hand.clone());

    let res: u64 = handbids
        .iter()
        .enumerate()
        .map(|(i, HandBid { hand: _, bid })| ((i as u64) + 1) * bid)
        .sum();
    println!("{res}");

    Ok(())
}

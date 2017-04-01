extern crate rand;

use rand::Rng;
use std::env;

const SUIT_NAMES : [&'static str; 4] = ["wands", "coins", "cups", "swords"];
const CARD_NAMES : [&'static str; 14] = ["ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "jack", "knight", "queen", "king"];
const TRUMP_NAMES : [&'static str; 22] = ["the fool", "the magician", "the high priestess", "the empress", "the emperor", "the hierophant", "the lovers", "the chariot", "strength", "the hermit", "wheel of fortune", "justice", "the hanged man", "death", "temperance", "the devil", "the tower", "the star", "the moon", "the sun", "judgement", "the world"];

fn print_card_name(tarot_number : usize) {
    if tarot_number < 14*4 {
        let suit_number : usize = tarot_number/14;
        let card_number : usize = tarot_number%14;
        print!("the {} of {}",
                 CARD_NAMES[card_number],
                 SUIT_NAMES[suit_number]);
    }
    else {
        let trump_number : usize = tarot_number - 4*14;
        print!("{}", TRUMP_NAMES[trump_number]);
    }
}

fn print_cards(num_cards : u32, deck : &mut Vec<usize>) {
    let deck_size = deck.len() as u32;
    let num_cards = if num_cards > deck_size + 1 { deck_size + 1 } else { num_cards };
    if deck_size == 0 {
        println!("There are no more cards to draw.");
    }
    else if num_cards < 1 {
        println!("You draw no cards.");
    }
    else {
        print!("You draw ");
        for x in 0..num_cards {
            if x == num_cards - 1 {
                if num_cards > 2 {
                    print!(", and ");
                }
                else if num_cards > 1 {
                    print!(" and ");
                }
            }
            else if x > 0 {
                print!(", ");
            }
            if deck.len() == 0 {
                print!("there are no more cards to draw");
                break;
            }
            let random_draw : usize = rand::thread_rng().gen_range(0, deck.len());
            let tarot_number = deck.swap_remove(random_draw);
            print_card_name(tarot_number);
        }
        println!("!");
    }
}

fn initialize_deck() -> Vec<usize>{
    let mut deck = vec!();
    for x in 0..78 {
        deck.push(x);
    }
    deck
}

fn main() {
    let mut remaining_deck = initialize_deck();
    if env::args().len() == 1 {
        print_cards(1, &mut remaining_deck);
    }
    else {
        for x in env::args().skip(1) {
            match x.trim().parse::<u32>() {
                Ok(num) => print_cards(num, &mut remaining_deck),
                Err(_) => println!("Please give positive integers as arguments."),
            };
        }
    }
}

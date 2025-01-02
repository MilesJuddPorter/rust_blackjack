use std::io::stdin;
use rand::prelude::*;
use itertools::Itertools;


struct Deck {
    cards: Vec<Card>,
}

struct Card {
    value: u16,
    suit: String,
    name: String,
}

struct Player {
    name: String,
    balance: u16,
}

struct Hand {
    cards: Vec<Card>,
    value: u16,
}

impl Hand {
    fn new() -> Self {
        Self {
            cards: Vec::new(),
            value: 0,
        }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.update_value();
    }

    fn update_value(&mut self) {
        let mut tempValue = 0;
        let mut hasAce = false;
        for card in &self.cards {
            if card.value == 11 {
                hasAce = true;
            }
            tempValue += card.value as u16;
        }
        if hasAce && tempValue > 21 {
            tempValue -= 10;
        }
        self.value = tempValue;
    }

    fn display_hand(&self, isDealer: bool) {
        let startVal = if isDealer { 1 } else { 0 };
        // let handValue = if isDealer {self.cards[1].value} else {self.value};
        let card_names = self.cards[startVal..].iter().map(|card| &card.name).join(", ");
        println!("[{}]", card_names);
    }
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        let suits = ["♥", "♦", "♣", "♠"];
        for suit in suits {
            for ii in 2..15 {
                let name = match ii {
                    2..=10 => ii.to_string(),
                    11 => "J".to_string(),
                    12 => "Q".to_string(),
                    13 => "K".to_string(),
                    14 => "A".to_string(),
                    _ => "".to_string(),
                };
                cards.push(Card {
                    value: if ii == 14 {11} else if ii > 10 { 10 } else { ii },
                    suit: suit.to_string(),
                    name: format!("{}{}", name, suit),
                });
            }
        }
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}


fn main() {
    println!("Welcome to the blackjack game!");
    // println!("Enter your name please");
    // let mut name = String::new();
    // stdin().read_line(&mut name).unwrap();
    // let name = name.trim();
    let name = String::from("Miles");
    let mut player = Player {
        name: name.to_string(),
        balance: 100
    };

    println!("Hello, {}, you are starting with ${} and $25 bets", player.name, player.balance.to_string());

    let mut deck = Deck::new();

    let mut handCount: u16 = 0;

    while player.balance > 0 {
        handCount += 1;
        println!("\n--------------------------------");
        println!("Hand {}", handCount);
        println!("You have ${}\n", player.balance);
        startHand(&mut deck, &mut player);
        println!("--------------------------------\n");
    }

    println!("You have no more money");
}

fn startHand(deck: &mut Deck, player: &mut Player) {
    let mut dealer_hand = Hand::new();
    let mut player_hand = Hand::new();

    dealer_hand.add_card(deck.draw().unwrap());
    dealer_hand.add_card(deck.draw().unwrap());
    player_hand.add_card(deck.draw().unwrap());
    player_hand.add_card(deck.draw().unwrap());

    if dealer_hand.value == 21 {
        if player_hand.value == 21 {
            println!("Both have blackjack");
            println!("[Push]");
            return;
        } else {
            println!("Dealer has blackjack");
            println!("[Dealer wins]");
            player.balance -= 25;
            return;
        }
    } else if player_hand.value == 21 {
        println!("You have blackjack");
        println!("[You win]");
        player.balance += 25 * (3/2);
        return;
    }

    println!("\nDealer's Hand:");
    dealer_hand.display_hand(true);

    println!("\nYour Hand:");
    player_hand.display_hand(false);
    playerTurn(deck, player, &mut dealer_hand, &mut player_hand);
}

fn playerTurn(deck: &mut Deck, player: &mut Player, dealer_hand: &mut Hand, player_hand: &mut Hand) {
    let mut player_choice = String::new();
    println!("Would you like to hit or stand? (h/s)");
    stdin().read_line(&mut player_choice).unwrap();
    let player_choice = player_choice.trim();
    if player_choice == "h" {
        player_hand.add_card(deck.draw().unwrap());
        player_hand.display_hand(false);
        if player_hand.value > 21 {
            println!("You bust");
            println!("[Dealer wins]");
            player.balance -= 25;
            return;
        } else {
            playerTurn(deck, player, dealer_hand, player_hand);
        }
    } else if player_choice == "s" {
        println!("You stand");
        println!("Dealer shows:");
        dealer_hand.display_hand(false);
        finalizeHand(deck, player, dealer_hand, player_hand);
    }
}

fn finalizeHand(deck: &mut Deck, player: &mut Player, dealer_hand: &mut Hand, player_hand: &mut Hand) {
    while dealer_hand.value < 17 {
        dealer_hand.add_card(deck.draw().unwrap());
        println!("Dealer draws a card");
        dealer_hand.display_hand(false);
        println!("Dealer's hand value: {}", dealer_hand.value);
    }

    if dealer_hand.value > 21 {
        println!("Dealer bust");
        println!("[You win]");
        player.balance += 25;
        return;
    }
    if player_hand.value > dealer_hand.value {
        println!("You win");
        println!("[You win]");
        player.balance += 25;
        return;
    } else if player_hand.value < dealer_hand.value {
        println!("You lose");
        println!("[Dealer wins]");
        player.balance -= 25;
        return;
    } else {
        println!("[Push]");
        return;
    }
}
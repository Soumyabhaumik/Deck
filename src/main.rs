use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = vec![];

        let suites = ["Hearts", "Spades", "Domonds", "Clubs"];
        let values = [
            "Ace", "King", "Queen", "Jack", "Two", "Three", "Four", "Five", "Six", "Seven",
            "Eight", "Nine", "Ten",
        ];

        for suite in suites {
            for value in values {
                let card = format!("{} of {}", value, suite);
                cards.push(card);
            }
        }
        // let deck = Deck { cards };
        // return deck;
        /* type of return */
        // return Deck { cards };

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let cards = deck.deal(3);

    println!("Here is your hands: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);

    println!("Thank you Run Again!!😊");
}

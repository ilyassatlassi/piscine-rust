use rand::Rng;
#[derive(Debug, PartialEq, Eq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq, Eq)]

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..4);
        match choice {
            0 => Suit::Heart,

            1 => Suit::Diamond,

            2 => Suit::Spade,

            3 => Suit::Club,
            _ => unreachable!(),
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,

            2 => Suit::Diamond,

            3 => Suit::Spade,

            4 => Suit::Club,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(1..=13);
        match choice {
            1 => Rank::Ace,

            2 => Rank::King,

            3 => Rank::Queen,

            4 => Rank::Jack,
            _ => Rank::Number(choice),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,

            2 => Rank::King,

            3 => Rank::Queen,

            4 => Rank::Jack,
            _ => Rank::Number(value),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return true;
    }
    return false;
}
#[cfg(test)]
mod tests {
    use super::*;

    // We cannot truly test the randomness as there's no 100% accurate consistent way to prove through a predicate that it yields a truly random number

    #[test]
    fn test_winner() {
        let winner = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        for rank in 1..14 {
            for suit in 1..5 {
                let card = Card {
                    rank: Rank::translate(rank),
                    suit: Suit::translate(suit),
                };

                assert_eq!(winner_card(&card), card == winner);
            }
        }
    }
}
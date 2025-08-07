#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u32),
    pub p2: (String, u32),
    pub nb_games: u32,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u32) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u32)> {
        let target = ((self.nb_games as f32) / 2.).ceil() as u32;
        println!("{}", target);
        if self.p1.1 > self.p2.1 {
            return Some(&self.p1);
        } else if self.p1.1 < self.p2.1 {
            return Some(&self.p2);
        }
        None
    }

    pub fn update_score(&mut self, user_name: &str) {
        let target = ((self.nb_games as f32) / 2.).ceil() as u32;
        if self.nb_games == self.p1.1 + self.p2.1 {
            return;
        }
        if self.p1.0 == user_name && self.p1.1 != target {
            self.p1.1 += 1;
        } else if self.p2.0 == user_name && self.p1.1 != target {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        let id = self.id;
        format!("game deleted: id -> {id}")
        // todo!()
    }
}

#[inline]
fn games() -> [GameSession; 3] {
    [
        GameSession::new(0, "player1".to_owned(), "player2".to_owned(), 1),
        GameSession::new(1, "Alice".to_owned(), "Mark".to_owned(), 3),
        GameSession::new(2, "Jack".to_owned(), "Miller".to_owned(), 5),
    ]
}

#[test]
fn test_update_and_read() {
    let mut games = games();
    games[0].update_score("player1");
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[0].update_score("player2");
    // this will stay the same because the nb_games is 1 so if one
    // of the players wins just once it will no longer increment the score
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Miller");
    games[2].update_score("Miller");
    assert_eq!(games[2].read_winner(), None);

    games[2].update_score("Jack");
    assert_eq!(games[2].read_winner(), Some(&("Jack".to_owned(), 3)));
}

#[test]
fn test_stop_updating() {
    let mut games = games();
    games[0].update_score("player1");
    games[0].update_score("player1");
    assert_eq!(games[0].read_winner(), Some(&("player1".to_owned(), 1)));

    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    games[2].update_score("Jack");
    assert_eq!(games[2].read_winner(), Some(&("Jack".to_owned(), 3)));
}

#[test]
fn test_delete() {
    let game = GameSession::new(0, "Alice".to_owned(), "Mark".to_owned(), 3);
    let game1 = GameSession::new(23, "Jack".to_owned(), "Miller".to_owned(), 1);

    assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
    assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
}

#[test]
fn test_different_name() {
    let mut game = GameSession::new(0, "Alice".to_owned(), "Mark".to_owned(), 3);

    game.update_score("Mark");
    assert_eq!(game.read_winner(), Some(&("Mark".to_owned(), 1)));

    game.update_score("Miller");
    assert_eq!(game.read_winner(), Some(&("Mark".to_owned(), 1)));
}

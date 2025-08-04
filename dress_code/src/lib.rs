#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>,
) -> Outfit {
    let mut out = Outfit {
        jacket: Jacket::Black,
        hat: Hat::Baseball,
    };
    match formality_level {
        Some(num) => {
            if num > 0 {
                out.jacket = Jacket::White;
            } else {
                out.jacket = Jacket::Black
            }
        }
        None => out.jacket = Jacket::Flowers,
    }
    match invitation_message {
        Ok(_) => out.hat = Hat::Fedora,
        Err(_) => {
            out.hat = Hat::Snapback;
            if    out.jacket == Jacket::Flowers {
                out.hat = Hat::Baseball
            }
        }
    }
    out
}

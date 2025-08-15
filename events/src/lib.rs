use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        match self.position {
            Position::Bottom => {
                write!(f, "{}", "Bottom ,")?;
            }
            Position::Top => {
                write!(f, "{}", "Top ,")?;
            }

            _ => write!(f, "{}", "Center ,")?,
        };

        write!(f, "{}", self.size)?;

        write!(
            f,
            "{}",
            self.content
                .truecolor(self.color.0, self.color.1, self.color.2)
        )?;
        Ok(())
    }
}

use Event::*;

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
 match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(time_left) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    DurationInHours::from(time_left)
                ),
            },
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            },
        }
    }
}

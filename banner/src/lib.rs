use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    // expected public fields
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: "-".to_string() + &name.chars().next().unwrap().to_string(),
            long_hand: "--".to_string() + name,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let func: fn(&str, &str) -> Result<String, ParseFloatError> = self.flags[input];
        match func(argv[0], argv[1]) {
            Ok(arg) => Ok(arg),
            Err(_) => Err("invalid float literal".to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match a.parse::<f64>() {
        Ok(num) => match b.parse::<f64>() {
            Ok(num2) => Ok((num / num2).to_string()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match a.parse::<f64>() {
        Ok(num) => match b.parse::<f64>() {
            Ok(num2) => Ok((num % num2).to_string()),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

// use banner::*;
use std::{ sync::LazyLock};

static HANDLER: LazyLock<FlagsHandler> = LazyLock::new(|| {
    let mut handler = FlagsHandler {
        flags: HashMap::new(),
    };

    handler.add_flag(Flag::opt_flag("division", "divides two numbers"), div);
    handler.add_flag(
        Flag::opt_flag(
            "remainder",
            "gives the remainder of the division between two numbers",
        ),
        rem,
    );

    handler
});

#[test]
fn test_simple() {
    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "2.0"]), Ok("0.5".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["2.0", "2.0"]), Ok("0".to_owned()));
    }

    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("0.058039751318764134".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("12.323".to_owned())
        );
    }
}

#[test]
fn test_edge_cases() {
    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["a", "2.0"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["2.0", "f"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("inf".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("NaN".to_owned()));
    }
}
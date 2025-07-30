// use std::{error::Error, fmt::Display};

// #[derive(Debug)]
// pub enum ParseErr {
//     // expected public fields
//     Empty,
//     Malformed,
// }

// impl Display for ParseErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let err = match self {
//             ParseErr::Empty => "",
//             ParseErr::Malformed => "",
//         };
//         write!(f, "{}", err)
//     }
// }

// impl Error for ParseErr {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(self)
//     }
// }

// #[derive(Debug)]
// pub struct ReadErr {
//     // expected public fields
//     child_err: Box<dyn Error>,
// }

// impl Display for ReadErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let err = match self.child_err {
//             ParseErr::Empty => "",
//             // ParseErr::Malformed => "",
//         };
//         write!(f, "{}", err)
//     }
// }

// impl Error for ReadErr {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         Some(self)
//     }
// }

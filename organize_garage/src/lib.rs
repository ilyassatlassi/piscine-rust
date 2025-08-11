// use std::{fmt::Debug, ops::Add};
use std::fmt::Debug;
#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T> {
    pub left: Option<T>,

    pub right: Option<T>,
}

impl<T: std::ops::Add<Output = T> + Copy + Debug> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let Some(val) = self.left {
            if let Some(val2) = self.right {
                self.right = Some(val2 + val);
                self.left = None;
            } else {
                self.right = Some(val);
                self.left = None;
            }
        }
    }

    pub fn move_to_left(&mut self) {
        if let Some(val) = self.right {
            if let Some(val2) = self.left {
                self.left = Some(val2 + val);

                self.right = None;
            } else {
                self.left = Some(val);

                self.right = None;
            }
        }
    }
}

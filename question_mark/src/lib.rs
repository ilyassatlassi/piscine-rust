pub struct One {
    // expected public fields
    pub first_layer: Option<Two>,
}
pub struct Two {
    // expected public fields
    pub second_layer: Option<Three>,
}
pub struct Three {
    // expected public fields

    pub  third_layer: Option<Four>,
}
pub struct Four {
    // expected public fields
       pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        Some(self.first_layer?.second_layer?.third_layer?.fourth_layer?)

        // todo!()
    }
}
// use question_mark::*;

#[cfg(test)]
mod tests {
    use super::*; // Import One, create_nested, etc.

    fn create_nested(value: Option<u16>) -> One {
        One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: value,
                    }),
                }),
            }),
        }
    }

    #[test]
    fn test_value() {
        assert_eq!(create_nested(Some(1000)).get_fourth_layer(), Some(1000));
        assert_eq!(create_nested(None).get_fourth_layer(), None);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen = match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };
        let rh = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, RhFactor::Positive) | (RhFactor::Positive, RhFactor::Negative) => {
                true
            }
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };
        antigen && rh
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut recipients = Vec::new();

        // Generate all possible blood types
        let all_antigens = [Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let all_rhs = [RhFactor::Positive, RhFactor::Negative];
        for anti in &all_antigens {
            for rh in &all_rhs {
                let other = BloodType {
                    antigen: anti.clone(),
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&other) {
                    recipients.push(other);
                }
            }
        }
        recipients
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut recipients = Vec::new();

        // Generate all possible blood types
        let all_antigens = [Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let all_rhs = [RhFactor::Positive, RhFactor::Negative];
        for anti in &all_antigens {
            for rh in &all_rhs {
                let other = BloodType {
                    antigen: anti.clone(),
                    rh_factor: rh.clone(),
                };
                if other.can_receive_from(self) {
                    recipients.push(other);

                }
            }
        }
        recipients
    }
}

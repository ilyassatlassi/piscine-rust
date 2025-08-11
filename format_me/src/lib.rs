use std::fmt;

pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}
impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type, self.name, self.address, self.cap, self.state
        )?;
        Ok(())
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParkType::Garden => write!(f, "garden")?,
            ParkType::Forest => write!(f, "forest")?,
            ParkType::Playground => write!(f, "playground")?,
        }
        Ok(())
    }
}

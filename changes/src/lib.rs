#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
	pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_string(),
            brightness: 0
        }
	}
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for ligth in lights {
       if ligth.alias == alias {
          ligth.brightness = value 
       } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias() {
        let mut lights = (0..5)
            .map(|i| Light::new(&format!("light-{i}")))
            .collect::<Vec<_>>();

        let alias = "light-3";
        change_brightness(&mut lights, alias, 100);
        assert_eq!(lights[3].brightness, 100);
    }

    #[test]
    fn test_nonexistent_alias() {
        let mut lights = (0..5)
            .map(|i| Light::new(&format!("light-{i}")))
            .collect::<Vec<_>>();

        let copy = lights.clone();
        change_brightness(&mut lights, "light-6", 100);
        assert_eq!(copy, lights);
    }
}
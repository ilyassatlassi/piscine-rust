pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> String {
    return film.name.to_string();
}

pub fn take_film_name(film: Film) -> String {
    return film.name;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        assert_eq!(
            take_film_name(Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        );
    }
    #[test]
    fn test_only_print() {
        assert_eq!(
            read_film_name(&Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        )
    }
}
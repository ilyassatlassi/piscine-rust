pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    name: String,
    accessLevel: AccessLevel,
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> User {
        User {
            name,
            accessLevel: level,
        }
    }
    pub fn send_name(&self) -> Option<&str> {
        match self.accessLevel {
            AccessLevel::Guest => None,
            _ => Some(&self.name),
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: User is guest"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let guest_case = User::new("Michael".to_string(), AccessLevel::Guest);
        assert_eq!(
            check_user_name(&guest_case),
            (false, "ERROR: User is guest")
        );
    }

    #[test]
    fn test_ok() {
        let admin_case = User::new("Fatima".to_string(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_string(), AccessLevel::Normal);
        assert_eq!(check_user_name(&admin_case), (true, "Fatima"));
        assert_eq!(check_user_name(&normal_case), (true, "Sara"));
    }

    #[test]
    fn test_send_name() {
        let admin_case = User::new("Fatima".to_string(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_string(), AccessLevel::Normal);
        let guest_case = User::new("Michael".to_string(), AccessLevel::Guest);
        assert_eq!(admin_case.send_name(), Some("Fatima"));
        assert_eq!(normal_case.send_name(), Some("Sara"));
        assert_eq!(guest_case.send_name(), None);
    }
}
// use crate::OfficeWorker::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(value: &str) -> Self {
        let mut office = OfficeWorker {
            name: Default::default(),
            age: Default::default(),
            role: WorkerRole::Admin,
        };

        for val in value.split(",").enumerate() {
            if val.0 == 0 {
                office.name = val.1.to_string();
            }

            if val.0 == 1 {
                office.age = val.1.parse().unwrap();
            }
            if val.0 == 2 {
                office.role = WorkerRole::from(val.1);
            }
        }
        office
    }
}

impl From<&str> for WorkerRole {
    fn from(value: &str) -> Self {
        match value {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            _ => WorkerRole::Guest,
        }
    }
}

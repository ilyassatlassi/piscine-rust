pub fn initials(names: Vec<&str>) -> Vec<String> {
names.into_iter().map(|val| {
    match val.split_once(" "){
        Some((first, last)) => format!("{}. {}.", first.chars().next().unwrap(), last.chars().next().unwrap()),
        None => String::new(),
    }
}).collect()
}

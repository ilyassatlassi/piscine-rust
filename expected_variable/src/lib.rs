use convert_case::{Case, Casing};
use edit_distance::edit_distance;
pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {

    if !compare.chars().any(|c| c == '_' || c.is_uppercase()) {
        return None;
    } 

    // Some("".to_string())
    let compare = compare.to_lowercase();
    let expected = expected.to_lowercase();
    if !compare.is_case(Case::Camel) && !compare.is_case(Case::Snake) {
        println!("compare is not camel nor snake {}", compare);
        return None;
    }
    
    let dis = edit_distance(&compare, &expected);
    let per = (100. - (dis as f64 / expected.len() as f64) * 100.).round();
    // println!("{}", dis);
    if per >= 50. {
        Some(format!("{per}%"))
    } else {
        None
    }
}



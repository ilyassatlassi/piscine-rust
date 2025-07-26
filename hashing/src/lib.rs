use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let length_list = list.len() as f64;
    let mut res= 0.0; 
    for num in list {
        res += *num as f64;
    }
    res/ length_list
}


pub fn median(list: &[i32]) -> i32 {

    let length_list = list.len();
let  mut sorted_numbers = list.to_vec(); 
    sorted_numbers.sort();
    if length_list% 2 != 0 {
      return sorted_numbers[length_list/2]
    }
    (sorted_numbers[(length_list/2) - 1] + sorted_numbers[(length_list/2) ]) / 2
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map1 = HashMap::new();
    let mut res = 0;
    let mut max = 0;
    for num in list {
        let count = map1.entry(*num).or_insert(0);
        *count+=1;
    }

    for (key, value) in map1 {
        if value > max {
            max = value;
            res = key;
        }
    }

    // map1[&res]
    res

}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[inline]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_mean() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert!(approx_eq(mean(&v), 3.857142857142857));
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[4, 7, 5, 2, 5, 1, 3]), 4);
        assert_eq!(median(&[2, 1, 5, 2, 7, 4]), 3);
        assert_eq!(median(&[1, 7, 5, 5, 6, 4]), 5);
    }

    #[test]
    fn test_mode() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mode(&v), 5);
    }
}
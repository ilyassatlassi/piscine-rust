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
    for num in list {
        let count = map1.entry(num.clone()).or_insert(0);
        *count+=1;

    }
    for (key, value) in map1 {
        if value > res {
            res = key;
        }
    }

    // map1[&res]
    res

}
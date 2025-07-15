pub fn is_empty(v: &str) -> bool {
    if v == "" {
        return true;
    }
    return false;
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars()  {
       let ascii = c as u32; 
       if ascii > 127 {
          return false; 
       }
    }
    return  true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
      v.find(pat).unwrap()


}

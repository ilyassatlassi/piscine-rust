pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() -1  {
       for j in 1+ i..arr.len()  {
          if arr[i]> arr[j] {
              arr.swap(i,j );
          } 
       } 
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut v = [3, 2, 4, 5, 1, 7, 9, 8];
        let mut v_clone = v;

        v_clone.sort_unstable();
        bubble_sort(&mut v);

        assert_eq!(v, v_clone);
    }
}
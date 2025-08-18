 use std::collections::BTreeSet;
pub fn flatten_tree<T: ToOwned<Owned= T>>(tree: &BTreeSet<T>) -> Vec<T> {
    let mut res = Vec::new();
    for item in tree {
       res.push(item.to_owned()) ;
    }
    res
}

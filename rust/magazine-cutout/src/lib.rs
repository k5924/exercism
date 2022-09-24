// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magmap = HashMap::new();
    for i in magazine {
        let mut counter = magmap.entry(i).or_insert(0);
        *counter += 1;
    }
    let mut notemap = HashMap::new();
    for i in note {
        let mut counter = notemap.entry(i).or_insert(0);
        *counter += 1;
    }
    let mut cmp: bool = false;
    for i in notemap.keys() {
        if !magmap.contains_key(i) {
            cmp = false;
            break;
        } else if magmap.get(i) < notemap.get(i) {
            cmp = false;
            break;
        } else {
            cmp = true;
        }
    }
    cmp
}

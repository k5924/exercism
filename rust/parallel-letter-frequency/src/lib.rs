use std::cmp::min;
use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();
    if input.is_empty() {
        return result;
    }
    let input = input.join("").to_ascii_lowercase();
    if input.len() == 0 {
        return result;
    }
    let mut input_it = input.chars();
    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut worker_length = (input.len() / real_worker_count).max(1);
    if worker_length * real_worker_count < input.len() {
        worker_length = worker_length + 1;
    }
    for _ in 0..real_worker_count {
        let chunk = input_it.by_ref().take(worker_length).collect::<String>();
        let t = thread::spawn(move || {
            let mut res = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    res.entry(c).and_modify(|val| *val += 1).or_insert(1);
                }
            });
            res
        });
        thread_pool.push(t);
    }
    for th in thread_pool {
        let answer = th.join().unwrap();
        for (key, val) in answer {
            result.entry(key).and_modify(|v| *v += val).or_insert(val);
        }
    }
    result
}

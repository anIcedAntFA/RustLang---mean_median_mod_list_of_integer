// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
fn main() {
    let list_of_int = vec![9, 1, 6, 7, 3, 5, 6, 19, 5, 6, 4, 56, 64, 96, 97];
    println!("{:?}", list_of_int);

    match calc_mean(&list_of_int) {
        Some(avg) => { println!("mean: {}", avg) },
        None => { println!("mean unavailable for {:?}", list_of_int); }
    }
    println!("");

    match calc_median(&list_of_int) {
        Some(med) => { println!("median: {}", med) },
        None => { println!("median unavailable for {:?}", list_of_int); }
    }
    println!("");

    match calc_mode(&list_of_int) {
        Some(mode) => println!("mode: {}", mode),
        None => { println!("mode unavailable for {:?}", list_of_int); }
    }
}

// Calculate Mean
fn calc_mean(list_of_int: &Vec<i32>) -> Option<f64>{
    let sum: i32 = list_of_int.iter().sum();
    let len = list_of_int.len();
    let avg = sum as f64 / len as f64;

    println!("sum: {}", sum);
    println!("len: {}", len);

    match avg.is_nan() {
        true => None,
        false => Some(avg),
    }
}

// Calculate Median
fn calc_median(list_of_int: &Vec<i32>) -> Option<f64> {
    // Sort list of integer
    let mut sorted_list = list_of_int.clone();
    sorted_list.sort();
    println!("sorted_list: {:?}", sorted_list);

    let mid_len = sorted_list.len() / 2;
    println!("mid_len: {}", mid_len);
    
    match sorted_list.get(mid_len) { 
        Some(mid_num) => {
            println!("mid_num: {}", mid_num);

            let mut med = *mid_num as f64;
            
            if sorted_list.len() % 2 == 0 {
                med += sorted_list[mid_len-1] as f64;
                med = med / 2.0;
            }
            return Some(med);
        }
        _ => None
    }
}

// Calculate Mode
fn calc_mode(list_of_int: &Vec<i32>) -> Option<i32> {
    // add keys to a hashmap and keep counts as values
    let mut times = HashMap::new();
    
    for num in list_of_int {
        let count = times.entry(num).or_insert(0);
        *count += 1;
    };
    println!("times: {:?}", times);

    let mut highest_count = 0;
    let mut highest_key = 0;
    let mut valid = true;

    for (key,value) in times {
        if value > highest_count {
            highest_count = value;
            highest_key = *key;
            valid = true;
        }
        else if value == highest_count {
            valid = false;
        }

        println!("key: {} and value: {}", key, value);
        println!("highest_count: {}", highest_count);
        println!("highest_key: {}", highest_key);
        println!("valid: {}\n", valid);
    }

    match highest_count >= 0 && valid == true {
        true => Some(highest_key),
        false => None,
    }   
}
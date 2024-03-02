use std::collections::HashMap;

fn main() {
    let mut mean_value = 0;
    let mut median_index = 0;
    let mut mean_map = HashMap::new();
    let mut mean: Option<i32> = None;
    let mut median: Option<i32> = None;
    let mut integers = vec![2, 3, 9, 2, 9, 10, 29];

    // finding the median

    integers.sort();
    let len_integers = integers.len();
    if len_integers % 2 == 0 {
        median_index = len_integers / 2
    } else {
        median_index = (len_integers + 1) / 2
    }

    median = Some(integers[median_index]);

    // finding the mean
    for i in &integers {
        let count = mean_map.entry(i).or_insert(0);
        *count += 1;
    }

    for (k, v) in &mean_map {
        if *v > mean_value {
            mean_value = *v
        }
    }

    mean = Some(mean_value);

    // output
    println!(
        "The mean value is {:?}, and the median is {:?}",
        mean.unwrap_or(0),
        median.unwrap_or(0)
    );
}

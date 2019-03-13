pub mod math {
    use std::collections::HashMap;
    
    pub fn median(list: &Vec<i32>) -> f32 {
        let mut sortlist = list.clone();
        sortlist.sort();
        if sortlist.len() % 2 != 0 {
            sortlist[list.len() / 2] as f32
        } else {
            (sortlist[list.len() / 2] + sortlist[(list.len() / 2)+1]) as f32 / 2.0
        }
    }
    pub fn mode(list: &Vec<i32>) -> i32 {
        let mut nums = HashMap::new();
        for i in list {
            *nums.entry(i).or_insert(0) += 1;
        }
        let mut largest = list[0];
        for (key, _) in &nums {
            match nums[key] {
                x if x > nums[&largest] => largest = **key,
                _ => ()
            }
        }
        largest
    }
    pub fn average(list: &Vec<i32>) -> f32 {
        list.into_iter().sum::<i32>() as f32 / list.len() as f32
    }
}



fn main() {
    let list = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];

    println!("Median: {}", math::median(&list));
    println!("Mode: {}", math::mode(&list));
    println!("Average: {}", math::average(&list));
}

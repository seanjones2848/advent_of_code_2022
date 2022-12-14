use std::{
    fs,
    collections::HashSet,
};

fn main() {
    let input = fs::read_to_string("puzzle_input")
        .expect("This should be able to read in the file");

    let mut sum = 0;
    for i in input.split("\n") {
        if i == "" { continue; }
        let mut ranges = i.split(",");
        let mut range_1 = ranges.next().unwrap().split("-").map(|x| x.parse::<i32>().unwrap());
        let mut range_2 = ranges.next().unwrap().split("-").map(|x| x.parse::<i32>().unwrap());
        let set_1: HashSet<i32> = (range_1.next().unwrap()..=range_1.next().unwrap()).collect::<Vec<i32>>().into_iter().collect();
        let set_2: HashSet<i32> = (range_2.next().unwrap()..=range_2.next().unwrap()).collect::<Vec<i32>>().into_iter().collect();
        let intersect = set_1.intersection(&set_2).collect::<Vec<&i32>>();
        if intersect.len() > 0 {
            sum += 1
        }
    }
    println!("{sum}");
}

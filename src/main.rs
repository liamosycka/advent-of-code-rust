use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_list_vec: Vec<i32> = Vec::new();
    let mut right_list_vec: Vec<i32> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./src/day1_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let v1: Vec<i32> = line
                .split("  ")
                .map(|x| x.trim().parse::<i32>().expect("The string is not a number"))
                .collect();
            left_list_vec.push(v1[0]);
            right_list_vec.push(v1[1]);
        }
    }

    //sort vecs ascending
    left_list_vec.sort();
    right_list_vec.sort();

    let mut total_distance: i32 = 0;
    let mut similarity_score: i32 = 0;
    //with iter, we can still use the vector afterwards
    //with into_iter, we cant, the reference is borrowed.
    for (i, n) in left_list_vec.into_iter().enumerate() {
        let val_l1: i32 = n;
        let val_l2: i32 = right_list_vec[i];

        total_distance += (val_l1 - val_l2).abs();

        //the filter returns a reference to the reference of each item,
        //thats why its necessary the double dereference (**) to access the value.
        let q_ocurrences: i32 = right_list_vec
            .iter()
            .filter(|x| **x == val_l1)
            .count()
            .try_into()
            .unwrap();
        similarity_score += val_l1 * q_ocurrences;
    }

    println!("Total distance ==> {total_distance}"); //result 1889772
    println!("Similarity score ==> {similarity_score}"); //result 23228917
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

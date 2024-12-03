use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

fn main() {
    let part1 = false;
    let mut sum: i32 = 0;
    let mut cur_sum: i32 = 0;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
           let v: Vec<&str> = line.split("   ").collect();
           v1.push(v[0].parse::<i32>().unwrap());
           v2.push(v[1].parse::<i32>().unwrap());
        }
    }
    v1.sort();
    v2.sort();

    for i in 0..v1.len() {
        if part1
        {
            sum += (v1[i] - v2[i]).abs();
        }
        else if !part1
        {
            if i == 0 || v1[i] != v1[i - 1]
            {
                cur_sum = 0;
                for elem in &v2
                {
                    match v1[i].cmp(elem) {
                        Ordering::Less => continue,
                        Ordering::Equal => cur_sum += v1[i],
                        Ordering::Greater => (),
                    }
                }

                sum += cur_sum;
            }
            else 
            {
                sum += cur_sum;    
            }
        }

    }

    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut count_safe = 0;
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let mut v: Vec<i32> = line.split(" ").map(|s: &str| s.parse::<i32>().unwrap()).collect();
            
            let sign = (v[0] - v[1]).signum(); // just set it here, saves us hassle
            let mut safe = true;
            for i in 0..v.len() - 1
            {
                let result: i32 = compare(v[i], v[i+1]);
                if result == 0 || result != sign
                {
                    if i == 0
                    {
                        // could be 0 or 1 that's bad
                        let removed = v.remove(0);
                        if !try_removed(&v)
                        {
                            v.insert(0, removed);
                            v.remove(1);
                            if !try_removed(&v)
                            {
                                safe = false;
                            }
                        }
                        break;
                    }
                    else if i == 1
                    {
                        // could be 0, 1, or 2 that's bad
                        let mut removed = v.remove(0);
                        if !try_removed(&v)
                        {
                            v.insert(0, removed);
                            removed = v.remove(1);
                            if !try_removed(&v)
                            {
                                v.insert(1, removed);
                                v.remove(2);
                                if !try_removed(&v)
                                {
                                    safe = false;
                                }
                            }
                        }
                        break;
                    }
                    else {
                        v.remove(i + 1);
                        if !try_removed(&v)
                        {
                            safe = false;
                        }
                        break;
                    }
                }
            }

            if safe {
                count_safe += 1;
            }
        }
    }

    println!("{}", count_safe);
}

fn try_removed(v: &[i32]) -> bool
{
    let sign = (v[0] - v[1]).signum(); // just set it here, saves us hassle
    for i in 0..v.len() - 1
    {
        let result: i32 = compare(v[i], v[i+1]);
        if result == 0 || result != sign
        {
            return false;
        }
    }

    true
}

// -1 = dec, 1 = inc, 0 = diff not in range
fn compare(num1: i32, num2: i32) -> i32
{
    let diff = num1 - num2;
    if diff.abs() >= 1 && diff.abs() <= 3
    {
        return diff.signum();
    }

    0
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
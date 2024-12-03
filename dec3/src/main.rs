use std::fs;
use regex::Regex;

fn main() {
    let part1 = false;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\)())|(don't\(\)())").unwrap();
    let text = fs::read_to_string("input.txt").expect("file doesn't exist");
    
    let mut sum: i32;

    if part1
    {
        sum = re.captures_iter(&text)
            .map(|c|c.extract().1)
            .fold(0, |acc, x: [&str; 2]| -> i32 {
                acc + x[0].parse::<i32>().unwrap() * x[1].parse::<i32>().unwrap()
            });
    }
    else 
    {
        sum = 0;
        let mut enabled = true;
        for [param1, param2] in re.captures_iter(&text).map(|c|c.extract().1)
        {
            if param1.starts_with("don't()")
            {
                enabled = false;
            }
            else if param1.starts_with("do()")
            {
                enabled = true;
            }
            else if enabled
            {
                sum += param1.parse::<i32>().unwrap() * param2.parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", sum);
}
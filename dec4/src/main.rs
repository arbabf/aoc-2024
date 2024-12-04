use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let part1= false;

    let mut v: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        lines.map_while(Result::ok).for_each(|line| {
            v.push(line);
        });
    }
    
    // shadow v, we don't need the old one anymore
    let v: Vec<&[u8]> = v.iter().map(|e|e.as_bytes()).collect();

    let mut total = 0;

    for i in 0..v.len()
    {
        for j in 0..v[i].len()
        {
            if part1 && v[i][j] == b'X'
            {
                total += find_xmas_p1(&v, i, j);
            }
            else if !part1 && v[i][j] == b'M'
            {
                total += find_xmas_p2(&v, i, j);
            }
        }
    }

    println!("{}", total);
}

fn find_xmas_p1(v: &[&[u8]], i: usize, j: usize) -> i32
{
    let mut xmas_count = 0;
    // rotations work clockwise in 8 directions starting at 0 for directly up, 1 up-right etc
    for rot in 0i32..=7 
    {
        if find_xmas(v, i, j, rot, b"MAS")
        {
            xmas_count += 1;
        }
    }
    
    xmas_count
}

fn find_xmas_p2(v: &[&[u8]], i: usize, j: usize) -> i32
{
    let mut xmas_count = 0;
    // rotations work clockwise in 8 directions starting at 0 for directly up, 1 up-right etc
    for rot in [1, 3, 5, 7] 
    {
        let chars = [b'A', b'S'];
        let mas_found = find_xmas(v, i, j, rot, &chars);

        if mas_found
        {
            // we can exploit the fact that we check sequentially by only checking mas'es down or to the right
            // as a result our array indexes will always work since a mas exists, guaranteeing that
            // there exists at least 2 more elements to index into from a given m
            // xmases also come in pairs only (otherwise we would have SAS or MAM), meaning we can end searches early
            // we can skip rot 7 since all matches from rot 7 are bottom-right, no more matches can be found there

            if rot == 1 && v[i][j + 2] == b'M'
            {
                // bottom-left element, only check right, rot 7
                if find_xmas(v, i, j + 2, 7, &chars)
                {
                    xmas_count += 1;
                    continue;
                }
            }
            else if rot == 3
            {
                // top-left, check right, rot 5, down, rot 1
                if (v[i][j + 2] == b'M' && find_xmas(v, i, j + 2, 5, &chars)) 
                || (v[i + 2][j] == b'M' && find_xmas(v, i + 2, j, 1, &chars))
                {
                    xmas_count += 1;
                    continue;
                }
            }
            else if rot == 5 && v[i + 2][j] == b'M'
            {
                // top-right, only check down, rot 7
                if find_xmas(v, i + 2, j, 7, &chars)
                {
                    xmas_count += 1;
                    continue;
                }
            }
        }
    }
    
    xmas_count
}

fn get_offset_from_rot(rot: i32) -> (i32, i32)
{
    let i_off = match rot {
        0 | 1 | 7 => -1,
        3 | 4 | 5 => 1,
        _ => 0,
    };

    let j_off = match rot {
        5..=7=> -1,
        1..=3 => 1,
        _ => 0,
    };

    (i_off, j_off)
}

// finds xmas and mas, bit of a misnomer but eh
fn find_xmas(v: &[&[u8]], i: usize, j: usize, rot: i32, chars: &[u8]) -> bool
{
    let (i_off, j_off) = get_offset_from_rot(rot);
    
    let mut i_mut = i;
    let mut j_mut = j;

    for char in chars
    {
        let result_i = i_mut as i32 + i_off;
        let result_j = j_mut as i32 + j_off;
        if result_i < 0 || result_i >= v.len() as i32
        || result_j < 0 || result_j >= v[i].len() as i32
        {
            return false;
        }
        i_mut = result_i as usize;
        j_mut = result_j as usize;

        if *char != v[i_mut][j_mut]
        {
            return false;
        }
    }
    true
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
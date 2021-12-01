use itertools::EitherOrBoth::Both;
use itertools::EitherOrBoth::Left;
use itertools::EitherOrBoth::Right;
use itertools::Itertools;

pub fn main() {
    println!("{}", count_increased(create_sums(adapt(include_str!("../input.txt")))));
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

//Converts the specified vector into a sums vector
fn create_sums(data: Vec<String>) -> Vec<String> {
    let result: Vec<String> = data
        .iter()
        .zip_longest(data.iter().skip(1)) // B
        .map(|a_b| {
            match a_b {
                Both(l, r) => l.parse::<i32>().unwrap() + r.parse::<i32>().unwrap(),
                Left(l) => l.parse::<i32>().unwrap(),
                Right(r) => r.parse::<i32>().unwrap(),
            }
        })
        .zip_longest(data.iter().skip(2)) // C
        .map(|ab_c| {
            match ab_c {
                Both(l, r) => l + r.parse::<i32>().unwrap(),
                Left(l) => l,
                Right(r) => r.parse::<i32>().unwrap(),
            }
        })
        .map(|i| i.to_string())
        .collect();
    return result;
}

//Counts how many times the measurements increased

pub fn count_increased(data: Vec<String>) -> usize {
    return data
        .iter()
        .rev()
        .zip_longest(data.iter().rev().skip(1))
        .map(|pair| {
            match pair {
                Both(l, r) => if l.parse::<u32>().unwrap() > r.parse::<u32>().unwrap() { 1 } else { -1 },
                Left(_) => 0,
                Right(_) => -1,
            }
        })
        .filter(|i| *i == 1)
        .count();
}

//Display the data as shown in the puzzle to visualize the calculation
#[allow(dead_code)]
fn display(data: Vec<String>) {
    let result: Vec<String> = data
        .iter()
        .rev()
        .zip_longest(data.iter().rev().skip(1))
        .map(|pair| {
            match pair {
                Both(l, r) => if l.parse::<u32>().unwrap() > r.parse::<u32>().unwrap() { l.to_owned() + " (increased)" } else if  l.parse::<u32>().unwrap() == r.parse::<u32>().unwrap() { l.to_owned() + " (no change)" } else { l.to_owned() + " (decreased)" },
                Left(l) => l.to_owned() + " (N/A - no previous measurement)",
                Right(r) => r.to_owned() + "(N/A - should never happen)",
            }
        })
        .collect();
    println!("{}", result.iter().rev().format("\n"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_increased() {
        let data: Vec<String> = vec![199,200,208,210,200,207,240,269,260,263].iter().map(|i| i.to_string()).collect();
        assert_eq!(5, count_increased(create_sums(data)));
    }
}

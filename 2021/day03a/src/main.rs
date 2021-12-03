use std::convert::TryInto;

pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let size = check_size(&data);
    let gama = gama_rate(size, data);
    let epsilon = epsilon_rate(size, gama);
    println!("{}", gama * epsilon);
}

//Adapt function that converts input data into a Vec<String>
pub fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

//Check that all input values have the same length and return that length
fn check_size(data: &Vec<String>) -> usize {
    return data.iter()
        .map(|l| l.chars().count())
        .reduce(|last, current | if last != current { panic!("All input values should have the same size!") } else { current })
        .expect("Input should contain at least one value!");
}

//Add the values of the two vectors
fn add(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    return a.iter().zip(b).map(|(l,r)| l + r).collect();
}

pub fn gama_rate(size: usize, data: Vec<String>) -> usize {
    let sums =  data.iter()
            .map(|l| l.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>())
            .fold(vec![0; size], |sum, x| { add(sum, x) });

    let binary_string = sums
            .iter()
            .map(|i| if i * 2 > data.len() { '1' } else { '0' }) 
            .collect::<String>();

    return usize::from_str_radix(&binary_string, 2).expect("Failed to convert binary string to decimal");
}

//Calculate the epsilon rate based on the size and gama rate.
//epislon + gama = 2^n-1 (cause they are complementary numbers)
//So, we can derive epsilon directly from gama.
pub fn epsilon_rate(size: usize, gama: usize) -> usize {
    let two: usize = 2;
    let n: u32 = size.try_into().expect("Failed to convert <usize> to <u32>!");
    return two.pow(n) - 1 - gama;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_size() {
        let data: Vec<String> = vec!["00100", "11110", "10110", "01010"].iter().map(|i| i.to_string()).collect();
        assert_eq!(5 , check_size(&data));
    }
    #[test]
    fn test_add() {
        assert_eq!(vec![1,1,1,1,1], add(vec![0,0,0,0,0], vec![1,1,1,1,1]));
        assert_eq!(vec![1,1,1,1,1], add(vec![1,0,1,0,1], vec![0,1,0,1,0]));
        assert_eq!(vec![2,2,2,2,2], add(vec![1,1,1,1,1], vec![1,1,1,1,1]));
        assert_eq!(vec![2,1,2,1,2], add(vec![1,0,1,0,1], vec![1,1,1,1,1]));
    }

    #[test]
    fn test_gama_rate() {
        let data: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"].iter().map(|i| i.to_string()).collect();
        assert_eq!(22 , gama_rate(5, data));
    }

    #[test]
    fn test_epsilon_rate() {
        assert_eq!(9 , epsilon_rate(5, 22));
        assert_eq!(9 , epsilon_rate(5, 23));
    }

}

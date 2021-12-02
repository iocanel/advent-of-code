pub fn main() {
    let (x, y, _) = process_commands(adapt(include_str!("../input.txt")));
    println!("{}", x*y)
}

//Adapt function that converts input data into a Vec<String>
pub fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

pub fn process_commands(data: Vec<String>) -> (i32, i32, i32) {
    return data.iter().map(|i| {
        let mut split = i.split(' ');
        let keyword = split.next().expect("Command should contain a keyword");
        let arg = split.next().expect("Command should contain an argument").parse::<i32>().unwrap();
        match keyword {
                "forward" => (arg, 0, 0),
                "down" => (0, 0, arg),
                "up" => (0, 0, -arg),
                _ => panic!("Unexpected command keyword!"),
            }
    }).fold((0,0,0), |sum, x| (sum.0 + x.0,
                               sum.1 + x.0 * sum.2 ,
                               sum.2 + x.2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_increased() {
        let data: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"].iter().map(|i| i.to_string()).collect();
        assert_eq!((15,60,10) , process_commands(data));
    }
}

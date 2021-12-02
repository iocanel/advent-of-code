pub fn main() {
    let (x, y) = process_commands(adapt(include_str!("../input.txt")));
    println!("{}", x*y)
}

//Adapt function that converts input data into a Vec<String>
pub fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

//Process the commands in the vector and return the vessel position
pub fn process_commands(data: Vec<String>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    return data.iter().map(|i| {
        let mut split = i.split(' ');
        let keyword = split.next().expect("Command should contain a keyword");
        let arg = split.next().expect("Command should contain an argument").parse::<i32>().unwrap();
        match keyword {
                "forward" => {x += arg; y += aim*arg},
                "down" => aim+=arg,
                "up" => aim-= arg,
                _ => panic!("Unexpected command keyword!"),
            }
        (x,y)
    }).last().expect("Data should contain at leat one command");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_increased() {
        let data: Vec<String> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"].iter().map(|i| i.to_string()).collect();
        assert_eq!((15,60) , process_commands(data));
    }
}

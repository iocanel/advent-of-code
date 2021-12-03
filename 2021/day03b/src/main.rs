pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let size = check_size(&data);
    let oxygen_generator_rating = oxygen_generator_rating(size, &data);
    let co2_scrubber_rating = co2_scrubber_rating(size, &data);
    println!(
        "Oxygen: {} CO2: {}. Product: {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

//Check that all input values have the same length and return that length
fn check_size(data: &Vec<String>) -> usize {
    return data
        .iter()
        .map(|l| l.chars().count())
        .reduce(|last, current| {
            if last != current {
                panic!("All input values should have the same size!")
            } else {
                current
            }
        })
        .expect("Input should contain at least one value!");
}

//Add the values of the two vectors
fn add(a: Vec<usize>, b: Vec<usize>) -> Vec<usize> {
    return a.iter().zip(b).map(|(l, r)| l + r).collect();
}

fn create_sums(size: usize, data: &Vec<String>) -> Vec<usize> {
    return data
        .iter()
        .map(|l| l.to_string())
        .map(line_to_digits)
        .fold(vec![0; size], |sum, x| add(sum, x));
}

fn most_common(size: usize, sums: &Vec<usize>) -> Vec<usize> {
    return sums
        .iter()
        .map(|i| if i * 2 >= size { 1 } else { 0 })
        .collect();
}

fn least_common(size: usize, sums: &Vec<usize>) -> Vec<usize> {
    return sums
        .iter()
        .map(|i| if i * 2 < size { 1 } else { 0 })
        .collect();
}

fn keep_matching_values(index: usize, values: &Vec<usize>, data: &Vec<String>) -> Vec<String> {
    return data
        .to_vec()
        .into_iter()
        .filter(|l| line_to_digits(l.to_string())[index] == values[index])
        .collect();
}

fn find_single_matching_value(size: usize, filter: fn(usize, &Vec<usize>) -> Vec<usize>, data: &Vec<String>) -> Option<String> {
    let mut result = data.to_vec();
    for i in 0..size {
        let sums = create_sums(size, &result);
        let values = filter(result.len(), &sums);
        result = keep_matching_values(i, &values, &result);
        if result.len() == 1 {
            return result.to_vec().into_iter().next();
        }
    }
    return Option::None;
}

fn line_to_digits(line: String) -> Vec<usize> {
    return line
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();
}

fn oxygen_generator_rating(size: usize, data: &Vec<String>) -> usize {
    return usize::from_str_radix(
        &find_single_matching_value(size, most_common, data).unwrap(),
        2,
    )
    .expect("Failed to convert binary string to decimal");
}

fn co2_scrubber_rating(size: usize, data: &Vec<String>) -> usize {
    return usize::from_str_radix(
        &find_single_matching_value(size, least_common, data).unwrap(),
        2,
    )
    .expect("Failed to convert binary string to decimal");
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_data() -> Vec<String> {
        return vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]
        .iter()
        .map(|i| i.to_string())
        .collect();
    }

    #[test]
    fn test_check_size() {
        let data = test_data();
        assert_eq!(5, check_size(&data));
    }

    #[test]
    fn test_add() {
        assert_eq!(vec![1, 1, 1, 1, 1], add(vec![0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1]));
        assert_eq!(vec![1, 1, 1, 1, 1], add(vec![1, 0, 1, 0, 1], vec![0, 1, 0, 1, 0]));
        assert_eq!(vec![2, 2, 2, 2, 2], add(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]));
        assert_eq!(vec![2, 1, 2, 1, 2], add(vec![1, 0, 1, 0, 1], vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_most_common() {
        let data = test_data();
        let sums = create_sums(5, &data);
        assert_eq!(vec![1, 0, 1, 1, 0], most_common(data.len(), &sums));
    }

    #[test]
    fn test_least_common() {
        let data = test_data();
        let sums = create_sums(5, &data);
        assert_eq!(vec![0, 1, 0, 0, 1], least_common(data.len(), &sums));
    }

    #[test]
    fn test_keep_mathcing_values() {
        let data = test_data();
        let sums = create_sums(5, &data);
        let most_common = most_common(data.len(), &sums);
        let least_common = least_common(data.len(), &sums);
        assert_eq!(vec!["11110", "10110", "10111", "10101", "11100", "10000", "11001"].sort(), keep_matching_values(0, &most_common, &data).sort());
        assert_eq!(vec!["00100", "01111", "00111", "00010", "01010"].sort(), keep_matching_values(0, &least_common, &data).sort());
    }

    #[test]
    fn test_oxygen_generator_rating() {
        assert_eq!(23, oxygen_generator_rating(5, &test_data()));
    }

    #[test]
    fn test_co2_scrubber_rating() {
        assert_eq!(10, co2_scrubber_rating(5, &test_data()));
    }
}

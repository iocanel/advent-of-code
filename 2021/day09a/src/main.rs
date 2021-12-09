pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let height_map = create_heightmap(data);
    println!("Risk:{}", calculate_risk(&height_map));
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn create_heightmap(data: Vec<String>) -> Vec<Vec<usize>> {
    return data.iter().map(|l| l.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
}

fn is_low_point(matrix: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    let above = if row > 0 { matrix[row - 1][col] } else { 10 };
    let below = if row < matrix.len() - 1 { matrix[row + 1][col] }  else { 10 };
    let previous = if col > 0 { matrix[row][col -1] } else { 10 };
    let next = if col < matrix[row].len() - 1 { matrix[row][col + 1] } else { 10 };

    let current = matrix[row][col];

    return current < above && current < below && current < previous && current < next;
}

fn calculate_risk(matrix: &Vec<Vec<usize>>) -> usize {
    let mut result: usize = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_low_point(matrix, i, j) {
                result += matrix[i][j] + 1;
            }
        }
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_risk() {
        let data = vec!["2199943210",
                        "3987894921",
                        "9856789892",
                        "8767896789",
                        "9899965678"].iter().map(|i| i.to_string()).collect();

        assert_eq!(15, calculate_risk(&create_heightmap(data)));
    }
}

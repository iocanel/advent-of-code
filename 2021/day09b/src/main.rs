pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let height_map = create_heightmap(data);
    let mut basins: Vec<usize> = calculate_basin_sizes(&height_map);
    basins.sort_by(|a, b| b.cmp(a));
    let result: usize = basins[0..3].to_vec().into_iter().reduce(|product, x| product * x).unwrap();
    println!("Result:{}", result); 
    
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn create_heightmap(data: Vec<String>) -> Vec<Vec<usize>> {
    return data.iter().map(|l| l.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
}

fn is_low_point(matrix: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    if row >= matrix.len() || col >= matrix[row].len() {
        return false;
    }

    let above = if row > 0 { matrix[row - 1][col] } else { 100 };
    let below = if row < matrix.len() - 1 { matrix[row + 1][col] }  else { 100 };
    let previous = if col > 0 { matrix[row][col -1] } else { 100 };
    let next = if col < matrix[row].len() - 1 { matrix[row][col + 1] } else { 100 };

    let current = matrix[row][col];
    return current < above && current < below && current < previous && current < next;
}

fn is_part_of_basin(matrix: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    let mut visited: Vec<usize> = Vec::new();
    return is_part_of_basin_v(matrix, row, col, &mut visited);
}

fn is_part_of_basin_v(matrix: &Vec<Vec<usize>>, row: usize, col: usize, visited: &mut Vec<usize>) -> bool {
    if row >= matrix.len() || col >= matrix[row].len() {
        return false;
    }
    
    if matrix[row][col] == 9  {
        return false;
    }

    let id: usize = row * matrix[row].len() + col;
    if visited.contains(&id) {
        return true;
    } else {
        visited.push(id);
    }

    if is_low_point(matrix, row, col) {
        return true;
    }

    return (row > 0 && is_part_of_basin_v(matrix, row - 1, col, visited)) ||
        (row + 1 < matrix.len() && is_part_of_basin_v(matrix, row + 1, col, visited)) ||
        (col > 0 && is_part_of_basin_v(matrix, row, col - 1, visited)) ||
        (col + 1 < matrix[row].len() && is_part_of_basin_v(matrix, row, col + 1, visited));
}

fn visit_basin(matrix: &Vec<Vec<usize>>, row: usize, col: usize, visited: &mut Vec<usize>) -> usize {
    let id: usize = row * matrix[row].len() + col;
    if is_part_of_basin(matrix, row, col) {
        if !visited.contains(&id) {
            visited.push(id);
        } else {
            return visited.len();
        }

        if row >= 1 && is_part_of_basin(matrix, row - 1, col) {
            visit_basin(matrix, row - 1, col, visited);
        }

        if is_part_of_basin(matrix, row + 1, col) {
            visit_basin(matrix, row + 1, col, visited);
        }

        if col >= 1 && is_part_of_basin(matrix, row, col - 1) {
            visit_basin(matrix, row, col - 1, visited);

        }
        if is_part_of_basin(matrix, row, col + 1) {
            visit_basin(matrix, row, col + 1, visited);
        }
    }
    return visited.len();
}

fn calculate_basin_sizes(matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if is_low_point(matrix, i, j) {
                let mut visited: Vec<usize> = Vec::new();
                result.push(visit_basin(matrix, i, j, &mut visited));
            }
        }
    }
    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_part_of_basin() {
        let data = vec!["2199943210",
                        "3987894921",
                        "9856789892",
                        "8767896789",
                        "9899965678"].iter().map(|i| i.to_string()).collect();

        let map = create_heightmap(data);

        assert_eq!(true, is_part_of_basin(&map, 0, 1));
        assert_eq!(true, is_part_of_basin(&map, 0, 0));
        assert_eq!(true, is_part_of_basin(&map, 1, 0));
        assert_eq!(true, is_part_of_basin(&map, 0, 9));
        assert_eq!(false, is_part_of_basin(&map, 1, 5));
    }

    #[test]
    fn test_visit_basins() {
        let data = vec!["2199943210",
                        "3987894921",
                        "9856789892",
                        "8767896789",
                        "9899965678"].iter().map(|i| i.to_string()).collect();

        let map = create_heightmap(data);
        assert_eq!(1, map[0][1]);

        let mut visited: Vec<usize> = Vec::new();
        assert_eq!(3, visit_basin(&map, 0 , 1, &mut visited));

        let mut visited: Vec<usize> = Vec::new();
        assert_eq!(9, visit_basin(&map, 0 , 9, &mut visited));

        let mut visited: Vec<usize> = Vec::new();
        assert_eq!(14, visit_basin(&map, 2 , 2, &mut visited));

        let mut visited: Vec<usize> = Vec::new();
        assert_eq!(9, visit_basin(&map, 4 , 6, &mut visited));
    }
}

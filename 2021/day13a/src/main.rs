pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let matrix = create_matrix(data); 
    display(&matrix);
    println!("Reult:{}", count_dots(&matrix));
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn create_matrix(data: Vec<String>) -> Vec<Vec<char>> {
    let coordinates: Vec<Vec<usize>> = data.iter()
        .filter(|l| l.contains(','))
        .map(|l| l.split(',').filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().expect("Failed to parse string to number!")).collect::<Vec<usize>>())
        .collect();

    let folds: Vec<(char, usize)> = data.into_iter()
        .filter(|l| l.contains('='))
        .map(|l| l.chars().skip("fold along ".len()).collect::<String>())
        .map(|l| l.split('=').map(|s| String::from(s)).collect::<Vec<String>>())
        .map(|v| (v[0].chars().next().unwrap(), v[1].parse::<usize>().unwrap()))
        .collect();

    let columns: usize = coordinates.iter().map(|e| e[0]).reduce(usize::max).expect("Failed to read the number of columns!") + 1;
    let rows: usize = coordinates.iter().map(|e| e[1]).reduce(usize::max).expect("Failed to read the number of rows!") + 1;

    let mut result: Vec<Vec<char>> = initialize_matrix(rows, columns);
    coordinates.iter().for_each(|e| result[e[1]][e[0]]='#');

    folds.iter()
        .take(1)
        .for_each(|f| {
            match f.0 {
                'x' =>  {
                    result = fold_left(&result, f.1);
                },
                'y' => {
                    result = fold_up(&result, f.1);
                }
                _ => panic!("Unexpected axis")
            }
        });
    return result;
}


fn project(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = a.len().max(b.len());
    let columns =  a[0].len().max(b[0].len());

    let mut result: Vec<Vec<char>> = initialize_matrix(rows, columns);
    (0..=rows).into_iter().for_each(|r| {
        (0..=columns).into_iter().for_each(|c| {
            if r < a.len() && c < a[r].len() && a[r][c] == '#' {
                result[r][c]='#';
            }
            if r < b.len() && c < b[r].len() && b[r][c] == '#' {
                result[r][c]='#';
            }
        });
    });
    return result;
}

fn preppend(matrix: &Vec<Vec<char>>, rows: usize, columns: usize) -> Vec<Vec<char>> {
    let total_rows = matrix.len() + rows;
    let total_columns = matrix[0].len() + columns;
    let mut result: Vec<Vec<char>> = initialize_matrix(total_rows, total_columns);
    (0..total_rows).into_iter().for_each(|r| {
        (0..total_columns).into_iter().for_each(|c| {
            if r < total_rows  - matrix.len() {
                result[r][c]='.';
            } else if c < total_columns - matrix[0].len() {
                result[r][c]='.';
            } else {
                result[r][c]=matrix[r - rows][c - columns];
            }
        });
    });
    return result;
}

fn fold_up(matrix: &Vec<Vec<char>>, row: usize) -> Vec<Vec<char>> {
    let mut upper = matrix.to_vec();
    let mut lower = upper.split_off(row).split_off(1);
    return project(&upper, &preppend(&turn_up(&lower), upper.len() - lower.len(), 0));
}

fn fold_left(matrix: &Vec<Vec<char>>, column: usize) -> Vec<Vec<char>> {
    let mut left = matrix.into_iter().map(|r| {
        let mut r_left = r.to_vec();
        let r_right = r_left.split_off(column).split_off(1);
        return r_left;
    }).collect::<Vec<Vec<char>>>();

    let mut right = matrix.into_iter().map(|r| {
        let mut r_left = r.to_vec();
        let r_right = r_left.split_off(column).split_off(1);
        return r_right;
    }).collect::<Vec<Vec<char>>>();

    return project(&left, &preppend(&turn_left(&right), 0, left[0].len() - right[0].len()));
}


fn turn_up(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return matrix.to_vec().into_iter().rev().collect();
}

fn turn_left(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return matrix.to_vec().into_iter()
        .map(|r| r.to_vec().into_iter().rev().collect())
       .collect();
}

fn count_dots(matrix: &Vec<Vec<char>>) -> usize {
    return matrix.into_iter()
        .map(|r| r.into_iter().filter(|i| *i == &'#').count())
        .reduce(|a,b| a + b).unwrap();
}

fn initialize_matrix(rows: usize, columns: usize) -> Vec<Vec<char>> {
    return (0..rows).into_iter().map(|r| (0..columns).into_iter().map(|c| '.').collect::<Vec<char>>()).collect();
}

fn display(matrix: &Vec<Vec<char>>) {
    matrix.iter().for_each(|r| {
        r.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initialize_matrix() {
        assert_eq!(vec![vec!['.']], initialize_matrix(1,1));
        assert_eq!(vec![vec!['.','.'], vec!['.','.']], initialize_matrix(2,2));
        assert_eq!(vec![vec!['.','.','.'], vec!['.','.','.']], initialize_matrix(2,3));
        assert_eq!(vec![vec!['.','.','.'], vec!['.','.','.'], vec!['.','.','.']], initialize_matrix(3,3));
    }

    #[test]
    fn test_turn_up() {
        let matrix = vec![vec!['#','.','.'], vec!['.','.','.'], vec!['.','.','.']]; 
        let expect = vec![vec!['.','.','.'], vec!['.','.','.'], vec!['#','.','.']]; 
        assert_eq!(expect, turn_up(&matrix))
    }

    #[test]
    fn test_turn_left() {
        let matrix = vec![vec!['#','.','.'], vec!['.','.','.'], vec!['.','.','.']]; 
        let expect = vec![vec!['.','.','#'], vec!['.','.','.'], vec!['.','.','.']]; 
        assert_eq!(expect, turn_left(&matrix))
    }

    #[test]
    fn test_prepend() {
        let matrix = vec![vec!['#','#','#'],
                          vec!['.','.','.'],
                          vec!['.','.','.']]; 

        let expect = vec![vec!['.','.','.'],
                          vec!['#','#','#'],
                          vec!['.','.','.'],
                          vec!['.','.','.']]; 

        assert_eq!(expect, preppend(&matrix, 1, 0));
        
        let matrix = vec![vec!['#','.','.'],
                          vec!['#','.','.'],
                          vec!['#','.','.']]; 

        let expect = vec![vec!['.','#','.','.'],
                          vec!['.','#','.','.'],
                          vec!['.','#','.','.']]; 

        assert_eq!(expect, preppend(&matrix, 0, 1));
    }


}


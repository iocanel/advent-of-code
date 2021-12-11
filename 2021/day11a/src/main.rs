pub fn main() {
    let data = adapt(include_str!("../input.txt"));
    let mut matrix = create_matrix(data);
    let mut result = 0;
    for i in 1..=100 {
        let step_flashes=next_step(&mut matrix);
        println!("Step: {}. Flashes: {}", i, step_flashes);
        display(&mut matrix);
        result+=step_flashes;
    }
    println!("Result: {}.", result);
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn create_matrix(data: Vec<String>) -> Vec<Vec<usize>> {
    return data.iter().map(|l| l.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect();
}


fn increase_adjuscent(matrix: &mut Vec<Vec<usize>>, row: usize, col: usize) {
    let neighbour_offset: Vec<(isize, isize)> = vec![(-1,-1), (-1, 0), (-1, 1),
                                                     (0, -1), (0, 1),
                                                     (1, -1), (1, 0), (1, 1)];
    let i = row as isize;
    let j = col as isize;
    neighbour_offset
        .iter()
        .for_each(|x|
             if i + x.0  >= 0 && i + x.0 < 10 && j + x.1 >= 0 && j + x.1 < 10 { //within bounds
                 matrix[(i + x.0) as usize][(j + x.1) as usize]+= 1;
                 if matrix[(i + x.0) as usize][(j + x.1) as usize] == 10 {
                     increase_adjuscent(matrix, (i + x.0) as usize, (j + x.1) as usize);
                 }
             });
}

fn next_step(matrix: &mut Vec<Vec<usize>>) -> usize  {
    let mut result: usize = 0;
    //1st pass
    for i in 0..10  {
        for j in 0..10  {
            matrix[i][j]+=1;
            if matrix[i][j] == 10 {
                increase_adjuscent(matrix, i, j);
            }
        }
    }

    for i in 0..10  {
        for j in 0..10  {
            if matrix[i][j] >= 10 {
                matrix[i][j] = 0;
                result+=1;
            }
        }
    }
    //2nd pass flush
    return result;
}

fn display(matrix: &mut Vec<Vec<usize>>)  {
    //1st pass
    for i in 0..10  {
        for j in 0..10  {
                print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_risk() {
    }
}

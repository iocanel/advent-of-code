pub fn main() {
    let data = adapt(include_str!("../input.txt"));

    let all_drawn = read_drawn(&data);
    let boards = read_boards(&data);

    let mut drawn: Vec<usize> = vec![];
    
    let mut last_board: Vec<Vec<usize>> = vec![];
    
    for i in 0..all_drawn.len() {
        drawn.push(all_drawn[i]);
        let remaining: Vec<Vec<Vec<usize>>> = boards.to_vec().into_iter()
            .filter(|b| !check_bingo(b, &drawn))
            .collect();

        if remaining.len() == 1 {
            last_board = remaining[0].to_vec();
        }

        if remaining.len() == 0 {
            println!();
            display_board(&last_board);
            println!("Drawn:{}", all_drawn[i]);
            println!("Sum:{}", sum_unmarked(&last_board, &drawn));
            println!("Product:{}", all_drawn[i] * sum_unmarked(&last_board, &drawn));
            break;
        }
    }
}

//Adapt function that converts input data into a Vec<String>
fn adapt(data: &str) -> Vec<String> {
    return data.lines().map(|i| i.to_string()).collect();
}

fn read_drawn(data: &Vec<String>) -> Vec<usize> {
    return data[0].split(",").map(|i| i.parse::<usize>().expect("Failed to parse string to number!")).collect();
}

fn read_board(data: &Vec<String>) -> Vec<Vec<usize>> {
    return data.iter()
        .map(|l| l.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<usize>().expect("Failed to parse string to number!")).collect::<Vec<usize>>())
        .collect();
}

fn read_boards(data: &Vec<String>) -> Vec<Vec<Vec<usize>>> {
    let clean_data: Vec<String> = data.into_iter()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty() && !l.contains(","))
        .collect::<Vec<String>>();

    return clean_data
        .chunks(5)
        .map(|c|  read_board(&c.to_vec()))
        .collect();
}

fn display_board(board: &Vec<Vec<usize>>) {
    board.iter()
        .for_each(|r| {
            r.iter().for_each(|c| print!("{} ", c));
            println!();
        });
    println!();
}

fn sum_unmarked(board: &Vec<Vec<usize>>, drawn: &Vec<usize>) -> usize {
    return board.iter()
        .map(|r| r.iter().fold(0, |sum, x| if drawn.contains(x) { sum } else { sum + x }))
        .fold(0, |sum, x| sum + x);
}

fn check_bingo(board: &Vec<Vec<usize>>, drawn: &Vec<usize>) -> bool {
    //1st pass rows
   if board.iter()
        .filter(|r| is_drawn(r, drawn))
        .count() != 0 {
            return true;
        }

    //2nd pass columns
    for c in 0..5 {
        let mut all_match = true;
        for r in 0..5 {
            if !drawn.contains(&board[r][c]) {
                all_match = false;
                break
            }
        }

        if all_match {
            return true;
        }
    }
    return false;
}

//returns true if all items in row_or_col have been drawn
fn is_drawn(row_or_col: &Vec<usize>, drawn: &Vec<usize>) -> bool {
    return row_or_col.iter()
        .filter(|i| !drawn.contains(i))
        .count() == 0;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_drawn() {
        assert_eq!(false, is_drawn(&vec![1,2,3,4,5], &vec![1,2,3,4]));
        assert_eq!(true, is_drawn(&vec![1,2,3,4,5], &vec![1,2,3,4,5]));
        assert_eq!(true, is_drawn(&vec![1,2,3,4,5], &vec![5,4,3,2,1]));
        assert_eq!(true, is_drawn(&vec![1,2,3,4,5], &vec![6,5,4,3,2,1]));

        assert_eq!(true, is_drawn(&vec![14,21,17,14,4], &vec![7,4,9,5,11,17,23,2,0,14,21,24]));
    }

   #[test]
    fn test_sum_unmarked() {
      let board = vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,20],
            vec![21,22,23,24,25],
        ];

        let drawn: Vec<usize> =vec![1,6,11,16,21];
        assert_eq!(270, sum_unmarked(&board, &drawn));
    }
  
    #[test]
    fn test_bingo() {
       let board = vec![
            vec![1,2,3,4,5],
            vec![6,7,8,9,10],
            vec![11,12,13,14,15],
            vec![16,17,18,19,20],
            vec![21,22,23,24,55],
        ];

        let drawn: Vec<usize> =vec![1,6,11,16,22];
        assert_eq!(false, check_bingo(&board,&drawn)); 

        let drawn: Vec<usize> =vec![4,9,13,18,23];
        assert_eq!(false, check_bingo(&board,&drawn)); 

       let board = vec![
            vec![14,21,17,24,4],
            vec![10,16,15,9,19],
            vec![18,8,23,26,20],
            vec![22,11,13,6,5],
            vec![2,0,12,3,7],
        ];

        let drawn: Vec<usize> = vec![7,4,9,5,11,17,23,2,0,14,21,24];
        assert_eq!(true, check_bingo(&board,&drawn)); 
    }
}

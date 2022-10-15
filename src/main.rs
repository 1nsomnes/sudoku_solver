fn main() {

}

fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    const EXPECTED_VALUES:[u8;9] = [1,2,3,4,5,6,7,8,9];

    let mut columns:[[u8;9]; 9] = [[0;9]; 9];

    for x in 0..9 {
        for y in 0..9 {
            columns[y][x] = puzzle[x][y];
        }
    }

    let mut boxes:[[[u8;9];3]; 3] = [[[0;9];3];3];
    //boxes[y][x][box_index]
    for start_y in (0..9).step_by(3) {
        for start_x in (0..9).step_by(3) {
            let mut index:usize = 0;
            for y in start_y..(start_y+3) {
                for x in start_x..(start_x+3) {
                    boxes[start_y/3][start_x/3][index] = puzzle[y][x];
                    index += 1;
                }
            }
        }
    }

    let find_missing_values = | list:&[u8; 9] | -> Vec<u8> {
        let value:Vec<u8> = EXPECTED_VALUES.clone().iter_mut().map(|d: &mut u8| if !list.contains(&d) { *d } else { 0 as u8 }).filter(|d| d != &0).collect();
        value
    };

    let mut row_missing_values:Vec<Vec<u8>> = puzzle.iter().map(|row| find_missing_values(row)).collect();
    let mut columns_missing_values:Vec<Vec<u8>> = columns.iter().map(|column| find_missing_values(column)).collect();
    let mut boxes_missing_values:[[Vec<u8>;3];3] = Default::default();

    for y in 0..3 {
        for x in 0..3 {
            boxes_missing_values[y][x] = find_missing_values(&boxes[y][x]);
        }
    }

    let remove_first_occurrence = |value: u8, list:&mut Vec<u8>| {
        for i in 0..list.len() {
            if list[i] == value {
                list.remove(i);
                break;
            }
        }
    };

    let mut intersection = |vec1:&mut Vec<u8>, vec2:&mut Vec<u8>| -> Vec<u8> {
        let mut result:Vec<u8> = Vec::new();
        for num in vec1 {
            if vec2.contains(num) && !result.contains(num) {
                result.push(num.clone());
            }
        }
        result
    };

    let mut attempt_solve = |coords:(usize,usize)|  -> Option<u8> {
        let row = &mut row_missing_values[coords.1];
        let column = &mut columns_missing_values[coords.0];
        let rbox = &mut boxes_missing_values[coords.1/3][coords.0/3];

        let intersections = intersection(rbox, &mut intersection(row, column));

        print!("\nIntersections at ({},{}):",coords.0,coords.1);
        intersections.iter().for_each(|i| print!("{},", i));

        if intersections.len() == 1 {

            let value = intersections[0];
            remove_first_occurrence(value, row);
            remove_first_occurrence(value, column);
            remove_first_occurrence(value, rbox);

            println!("{}", value);

            return Some(value);
        }
        None
    };

    fn is_solved(board:&[[u8; 9]; 9]) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if board[y][x] == 0 { return false; }
            }
        }
        true
    }

    while !is_solved(&puzzle) {
        for y in 0..9 {
            for x in 0..9 {
                if &puzzle[y][x] == &0 {
                    match attempt_solve((x, y)) {
                        Some(value) => puzzle[y][x] = value,
                        None => ()
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku;

    #[test]
    fn puzzle_test_1() {
        println!("\n\nTest 1:");
        let mut puzzle = [
            [6, 0, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }

    #[test]
    fn puzzle_test_2() {
        println!("\n\nTest 2:");

        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(puzzle, solution, "\nYour solution (left) did not match the correct solution (right)");
    }
}

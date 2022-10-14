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

    for start_y in (0..9).step_by(3) {
        for start_x in (0..9).step_by(3) {
            for y in start_y..(start_y+3) {
                for x in start_x..(start_x+3) {
                    for index in 0..9 {
                        boxes[start_y/3][start_x/3][index] = puzzle[y][x];
                    }
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

    for row in row_missing_values {
        //println!("Row:");
        for m_v in row {
            //println!("{}", m_v);
        }
    }


    todo!()
}

#[cfg(test)]
mod tests {
    use crate::sudoku;

    #[test]
    fn puzzle_test_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
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

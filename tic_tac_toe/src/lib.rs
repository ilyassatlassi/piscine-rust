

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let mut res = String::from("player ");

        if vertical('X', table) || horizontal('X', table) || diagonals('X', table) {
            res.push('X');
            res.push_str(" won");
            return res;
        }
        if vertical('O', table) || horizontal('O', table) || diagonals('O', table) {
            res.push('O');
            res.push_str(" won");
            return res;
        }

    res = String::from("tie");
    res
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    return  false;
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            if table[i][j] == player {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        }
        count = 0;
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            if table[j][i] == player {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        }
        count = 0;
    }
    return false;
}



#[cfg(test)]
mod tests {
    use super::*;

    const DIAGONAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('X', [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]),
        ('O', [['O', 'X', 'O'], ['X', 'O', 'O'], ['X', '#', 'O']]),
    ];

    const HORIZONTAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'O', 'O'], ['X', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'X', 'O'], ['O', 'O', 'O'], ['O', '#', 'X']]),
        ('X', [['O', 'X', 'O'], ['O', '#', 'O'], ['X', 'X', 'X']]),
    ];

    const VERTICAL_TESTS: &[(char, [[char; 3]; 3])] = &[
        ('O', [['O', 'X', 'O'], ['O', 'X', 'O'], ['O', '#', 'X']]),
        ('O', [['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']]),
        ('X', [['O', 'X', 'X'], ['O', 'X', 'X'], ['X', '#', 'X']]),
    ];

    const TIE_TESTS: &[[[char; 3]; 3]] = &[
        [['O', 'X', 'O'], ['O', 'X', 'O'], ['X', '#', 'X']],
        [['O', 'X', 'O'], ['X', 'X', 'O'], ['X', '#', 'X']],
    ];

    #[test]
    fn test_diagonal() {
        DIAGONAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(diagonals(p, t)));
    }

    #[test]
    fn test_horizontal() {
        HORIZONTAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(horizontal(p, t)));
    }

    #[test]
    fn test_vertical() {
        VERTICAL_TESTS
            .iter()
            .copied()
            .for_each(|(p, t)| assert!(vertical(p, t)));
    }

    #[test]
    fn test_tic_tac_toe() {
        [DIAGONAL_TESTS, HORIZONTAL_TESTS, VERTICAL_TESTS]
            .concat()
            .into_iter()
            .for_each(|(p, t)| assert_eq!(tic_tac_toe(t), format!("player {} won", p)));

        TIE_TESTS
            .iter()
            .copied()
            .for_each(|t| assert_eq!(tic_tac_toe(t), "tie"));
    }
}
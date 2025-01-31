use crate::{STATE_SIDE, STATE_SIZE};

fn elementary(a: &mut u32, b: &mut u32, d: &mut u32, offset: u32) {
    *a = a.wrapping_add(*b);
    *d ^= *a;
    let _ = d.rotate_left(offset);
}

fn quarter_round(row: [u32; STATE_SIDE]) -> [u32; STATE_SIDE] {
    let mut a = row[0];
    let mut b = row[1];
    let mut c = row[2];
    let mut d = row[3];
    elementary(&mut a, &mut b, &mut d, 16);
    elementary(&mut c, &mut d, &mut b, 12);
    elementary(&mut a, &mut b, &mut d, 8);
    elementary(&mut c, &mut d, &mut b, 7);
    row
}

fn column(state: [u32; STATE_SIZE], col: usize) -> [u32; STATE_SIDE] {
    let mut out_col = [0u32; STATE_SIDE];
    for row in 0..STATE_SIDE {
        out_col[row] = state[col + row * STATE_SIDE];
    }
    out_col
}

fn to_columns(state: [u32; STATE_SIZE]) -> [[u32; STATE_SIDE]; STATE_SIDE] {
    let mut columns = [[0u32; STATE_SIDE]; STATE_SIDE];
    for col in 0..STATE_SIDE {
        columns[col] = column(state, col);
    }
    columns
}

fn from_columns(columns: [[u32; STATE_SIDE]; STATE_SIDE]) -> [u32; STATE_SIZE] {
    let mut state = [0u32; STATE_SIZE];
    for col in 0..STATE_SIDE {
        for row in 0..STATE_SIDE {
            state[row + col * STATE_SIDE] = columns[row][col];
        }
    }
    state
}

fn diagonal(state: [u32; STATE_SIZE], offset: usize) -> [u32; STATE_SIDE] {
    let mut out_col = [0u32; STATE_SIDE];
    for ind in 0..STATE_SIDE {
        out_col[ind] = state[(offset + ind) % STATE_SIDE + ind * STATE_SIDE];
        #[cfg(test)]
        eprint!("{} ", (offset + ind) % STATE_SIDE + ind * STATE_SIDE);
    }
    #[cfg(test)]
    eprintln!("");
    out_col
}

fn to_diagonals(state: [u32; STATE_SIZE]) -> [[u32; STATE_SIDE]; STATE_SIDE] {
    let mut diagonals = [[0u32; STATE_SIDE]; STATE_SIDE];
    for offset in 0..STATE_SIDE {
        diagonals[offset] = diagonal(state, offset);
    }
    diagonals
}

fn from_diagonals(diagonals: [[u32; STATE_SIDE]; STATE_SIDE]) -> [u32; STATE_SIZE] {
    let mut state = [0u32; STATE_SIZE];
    for offset in 0..STATE_SIDE {
        for ind in 0..STATE_SIDE {
            state[(offset + ind) % STATE_SIDE + ind * STATE_SIDE] = diagonals[offset][ind];
        }
    }
    state
}

fn column_round(state: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    from_columns(to_columns(state).map(quarter_round))
}

fn diagonal_round(state: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    from_diagonals(to_diagonals(state).map(quarter_round))
}

fn full_round(state: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    diagonal_round(column_round(state))
}

#[cfg(test)]
mod test {
    use crate::{
        instructions::{from_columns, from_diagonals, to_columns, to_diagonals},
        STATE_SIZE,
    };

    const STATE: [u32; STATE_SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    #[test]
    fn from_to_columns() {
        assert_eq!(STATE, from_columns(to_columns(STATE)));
    }

    #[test]
    fn from_to_diagonals() {
        assert_eq!(
            STATE,
            from_diagonals(to_diagonals(STATE)),
            "{:?}",
            to_diagonals(STATE)
        );
    }
}

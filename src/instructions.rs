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

fn column_round(state: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    let updated = to_columns(state);
    from_columns(updated.map(quarter_round))
}

#[cfg(test)]
mod test {
    use crate::{
        instructions::{from_columns, to_columns},
        STATE_SIZE,
    };

    #[test]
    fn from_to_reciprocals() {
        let state: [u32; STATE_SIZE] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(state, from_columns(to_columns(state)));
    }
}

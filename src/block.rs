use crate::{instructions::full_round, STATE_SIDE, STATE_SIZE};

const C: [u32; STATE_SIDE] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

fn init_state(key: [u8; STATE_SIDE * STATE_SIZE], nonce: &str, counter: u32) -> [u32; STATE_SIZE] {
    assert!(nonce.len() <= 24);
    let mut state = [0u32; STATE_SIZE];

    state[..STATE_SIDE].copy_from_slice(&C); // constants

    state[STATE_SIDE..3 * STATE_SIDE].copy_from_slice(
        &key.chunks(4)
            .map(|chunk: &[u8]| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<u32>>(),
    ); // import key

    state[3 * STATE_SIDE] = counter; // block counter

    let nonce = &u128::from_str_radix(nonce, 16).unwrap().to_le_bytes()[..12];
    state[3 * STATE_SIDE + 1..].copy_from_slice(
        &nonce
            .chunks(4)
            .map(|chunk: &[u8]| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<u32>>(),
    );

    state
}

const N_ROUNDS: usize = 20;

fn update_state(mut state: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    let init = state.clone();
    for _ in 0..N_ROUNDS {
        state = full_round(state);
    }
    state
        .iter()
        .zip(init)
        .map(|(a, b)| a.wrapping_add(b))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

pub fn block(key: [u8; STATE_SIDE * STATE_SIZE], nonce: &str, counter: u32) -> [u32; STATE_SIZE] {
    update_state(init_state(key, nonce, counter))
}

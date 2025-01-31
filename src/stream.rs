use crate::{block::block, STATE_SIDE, STATE_SIZE};

pub const IV: u32 = 0;

fn pad(chunk: &[u8]) -> [u32; STATE_SIZE] {
    chunk
        .chunks(4)
        .map(|chnk: &[u8]| u32::from_le_bytes([chnk[0], chnk[1], chnk[2], chnk[3]]))
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

fn XOR(message: [u32; STATE_SIZE], salt: [u32; STATE_SIZE]) -> [u32; STATE_SIZE] {
    message
        .iter()
        .zip(salt)
        .map(|(m, s)| m ^ s)
        .collect::<Vec<u32>>()
        .try_into()
        .unwrap()
}

pub fn stream(in_stream: Vec<u8>, key: [u8; STATE_SIDE * STATE_SIZE], nonce: &str) -> Vec<u8> {
    let size = in_stream.len();
    let stream_blocks = in_stream.chunks(64).map(pad);
    //let b = random_range(..stream_blocks.len());
    let mut b = IV;
    let mut salted = stream_blocks
        .map(|message| {
            XOR(
                message,
                block(key, nonce, {
                    b += 1;
                    b
                }),
            )
        })
        .flat_map(|salted| {
            salted
                .iter()
                .flat_map(|word: &u32| u32::to_le_bytes(*word))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<u8>>();
    salted.truncate(size);
    salted
}

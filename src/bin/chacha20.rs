use std::fs;

use TD3::stream::stream;

fn main() {
    let mut args = std::env::args();
    let key = fs::read(
        &args
            .nth(1)
            .expect("First argument should be the path to the keyfile"),
    )
    .expect("Cannot read keyfile");

    let nonce = &args
        .nth(2)
        .expect("Second argument should a 24-hexadecimal string");

    let in_stream = fs::read(
        &args
            .nth(3)
            .expect("Third argument should be the path to the file to salt"),
    )
    .expect("Cannot read file");

    let _ = fs::write(
        &args
            .nth(4)
            .expect("Fourth argument should be the path to the file to write the salt to"),
        stream(
            in_stream,
            key.try_into()
                .expect("Keyfile should be exactly 32 bytes long"),
            nonce,
        ),
    )
    .expect("Could not write to file");
}

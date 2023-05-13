use wec::bec;

// Make sure the behavior of bec![] is the same as vec![] when creating empty
// vectors.

fn main() {
    let v = bec![];
}

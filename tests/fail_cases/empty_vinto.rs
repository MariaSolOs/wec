use wec::vinto;

// Make sure the behavior of vinto![] is the same as vec![] when creating empty
// vectors.

fn main() {
    let v = vinto![];
}

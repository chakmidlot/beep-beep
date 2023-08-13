mod code;
mod signaler;

use crate::signaler::Signaler;

fn main() {
    let signaler = Signaler::new(8_000, 100, 300, 700, 1_000);
    signaler.play("Rodio");
}

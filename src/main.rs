mod lotto;

use lotto::Lotto;

fn main() {
    let mut game = Lotto::new();
    game.start();
    game.print_lotto();
}

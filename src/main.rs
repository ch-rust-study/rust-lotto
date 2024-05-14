mod lotto;

use lotto::Lotto;

fn main() {
    let mut game = Lotto::new();
    game.start();
    game.print_lotto();
    game.input_win_number();
    game.input_bonus_number();
}

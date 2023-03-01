mod logic;
use logic::Explorers;



fn main() {
    let mut game = Explorers::new();
    game.start_game();
}
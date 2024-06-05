use desafios::desafio_um::calibration_values;
use crate::desafios::desafio_um::spelled_out_with_letters;
use crate::desafios::desafio_dois::{power_games, valid_games};
use crate::desafios::desafio_tres::engine_schematic;

pub mod desafios;
mod modelos;

fn main() {
    println!("{}", calibration_values(include_str!("dados/desafio_um.txt")));
    println!("{}", calibration_values(&*spelled_out_with_letters(include_str!("dados/desafio_um.txt"))));
    println!("{}", valid_games(include_str!("dados/desafio_dois.txt")));
    println!("{}", power_games(include_str!("dados/desafio_dois.txt")));
    engine_schematic()
}

use desafios::desafio_um_dia_um::calibration_values;
use crate::desafios::desafio_dois_dia_um::spelled_out_with_letters;
use crate::desafios::desafio_um_dia_dois::{power_games, valid_games};

pub mod desafios;
mod modelos;

fn main() {
    println!("{}", calibration_values(include_str!("dados/desafio_um_dia_um.txt")));
    println!("{}", calibration_values(&*spelled_out_with_letters(include_str!("dados/desafio_dois_dia_um.txt"))));
    println!("{}", valid_games(include_str!("dados/desafio_um_dia_dois.txt")));
    println!("{}", power_games(include_str!("dados/desafio_um_dia_dois.txt")));
}

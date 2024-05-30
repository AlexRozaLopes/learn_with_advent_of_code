use desafios::desafio_um_dia_um::calibration_values;
use crate::desafios::desafio_dois_dia_um::spelledOutWithLetters;

pub mod desafios;

fn main() {
    println!("{}", calibration_values(include_str!("dados/desafio_um_dia_um.txt")));
    println!("{}", calibration_values(&*spelledOutWithLetters(include_str!("dados/desafio_dois_dia_um.txt"))))
}

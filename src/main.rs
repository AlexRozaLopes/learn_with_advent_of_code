pub mod desafios;

fn main() {
    println!("{}", desafios::desafio_um::desafio_um(include_str!("./dados/desafio_um.txt")))
}

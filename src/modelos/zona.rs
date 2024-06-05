#[derive(Debug,Clone,PartialEq,Eq,)]
pub struct Zona<T> {
    pub elemento: T,
    pub posicao_inicial: i32,
    pub posicao_final: i32,
    pub linha: i32,
}

impl<T> Zona<T> {
    pub fn new(elemento: T, posicao_inicial: i32, posicao_final: i32, linha: i32) -> Self {
        Self { elemento, posicao_inicial, posicao_final, linha }
    }
}



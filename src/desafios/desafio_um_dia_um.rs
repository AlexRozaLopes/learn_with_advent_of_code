pub fn calibration_values(dados: &str) -> i32 {
    dados.lines().filter_map(|linha| {
        println!("{}", linha);
        let numeros_da_linha = || linha.chars().filter(char::is_ascii_digit);
        let primeiro_numero = numeros_da_linha().next().unwrap_or_else(|| "0".chars().next().unwrap());
        let ultimo_numero = numeros_da_linha().last().unwrap_or_else(|| primeiro_numero);
        println!("{}{}", primeiro_numero, ultimo_numero);
        format!("{primeiro_numero}{ultimo_numero}").parse::<i32>().ok()
    }).sum::<i32>()
}

#[test]
fn exemplo_desafio_um() {
    let soma = calibration_values("1abc2
pqr3stu8vwx
a1b2c3d4e5f
sevenine
treb7uchet");
    assert_eq!(142,soma)
}
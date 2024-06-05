
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


pub fn spelled_out_with_letters(dados: &str) -> String {
    let mut texto_formatado = String::new();
    dados.lines().for_each(|linhas| {
        texto_formatado.push_str(&linhas.replace("one", "on1e")
            .replace("two", "t2wo")
            .replace("three", "th3ree")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "si6x")
            .replace("seven", "se7ven")
            .replace("eight", "eig8ht")
            .replace("nine", "ni9ne"));
        texto_formatado.push('\n');

    });
    println!("{texto_formatado}");
    texto_formatado
}


#[test]
fn exemplo_desafio() {
    let soma = calibration_values("1abc2
pqr3stu8vwx
a1b2c3d4e5f
sevenine
treb7uchet");
    assert_eq!(142,soma)
}



#[test]
fn exemplo_desafio_parte_dois() {
    let texto_formatado = spelled_out_with_letters("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
    assert_eq!(281, calibration_values(&*texto_formatado))
}
use crate::desafios::desafio_um_dia_um::calibration_values;

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
fn exemplo_desafio_dois() {
    let texto_formatado = spelled_out_with_letters("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
    assert_eq!(281, calibration_values(&*texto_formatado))
}
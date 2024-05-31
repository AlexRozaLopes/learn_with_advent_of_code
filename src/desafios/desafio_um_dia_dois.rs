use std::ops::Add;
use crate::modelos::game::Game;

pub fn valid_games(dados: &str) -> i32 {
    let games = split_game(dados);
    games.iter().filter(|game|{
        game.red <= 12 && game.green <= 13 && game.blue <= 14
    }).map(|game| {
        println!("{game:?}");
        game.id
    }).sum()
}

fn split_game(dados: &str) -> Vec<Game> {

    let mut games: Vec<Game> = vec![];

    dados.lines().for_each(|linha| {
        let linha_formatada = linha.replace(":",",").replace(";",",");
        let game_formatado: Vec<&str> = linha_formatada.split(",").collect();

        let mut game = Game::new(0, 0, 0, 0);

        for game_conteudo in game_formatado {
            if game_conteudo.contains("Game") {
                game.id = game.id.add(
                    find_valor(game_conteudo)
                );
            }else if game_conteudo.contains("red") {
                game.red = game.red.add(
                    find_valor(game_conteudo)
                )
            }else if game_conteudo.contains("green") {
                game.green = game.green.add(find_valor(game_conteudo))
            } else {
                game.blue = game.blue.add(find_valor(game_conteudo))
            };

            //println!("{game_conteudo}");
            //println!("{game:?}");

        }
        games.push(game);
    });

    games
}

fn find_valor(game_conteudo: &str) -> i32 {
    game_conteudo.chars()
        .filter(char::is_ascii_digit)
        .map(|literal| literal.to_string())
        .reduce(|mut acc, e| {
            acc.push_str(&e);
           // println!("{acc}");
            acc
        }).unwrap().parse::<i32>().unwrap()
}


#[test]
fn exemplo_desafio() {
    let exemplo = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(8, valid_games(exemplo))
}
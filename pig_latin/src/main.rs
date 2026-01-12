/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: taewonki <taewonki@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/01/12 16:24:42 by taewonki          #+#    #+#             */
/*   Updated: 2026/01/12 17:15:52 by taewonki         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// pig latin 만들기
// 1. 첫글자가 자음이면 첫글자 맨 뒤로 보내고 -ay추가
// 2. 첫글자 모음이면 단어뒤에 바로 -hay 추가
// 3. 영문 아니면 그냥 그대로 냅둬

use std::io::{self, Write};

fn convert_pl(word: &str) -> String
{
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let first_char = match word.chars().next(){
        Some(c) => c,
        None => return String::new(),
    };

    if !first_char.is_ascii_alphabetic()
    {
        return word.to_string();
    }
    if vowels.contains(&first_char)
    {
        format!("{word}-hay")
    }
    else
    {
        let rest = &word[1..];
        format!("{}-{}ay", rest, first_char)
    }
}

fn main() {
    let mut input = String::new();

    loop
    {
        print!("Pig-latin maker : ");
        io::stdout().flush().unwrap();
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Read Error");

        if input.trim() == "exit"{
            break;
        }
        for word in input.trim().split_whitespace()
        {
            print!("{}", convert_pl(word));
        }
        println!();
    }
}

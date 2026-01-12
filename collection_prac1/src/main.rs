/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: taewonki <taewonki@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2026/01/12 15:00:15 by taewonki          #+#    #+#             */
/*   Updated: 2026/01/12 16:21:52 by taewonki         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

    // 프로젝트 목표
    // 1. 파라미터로 정렬 대상 배열을 받아서 정렬해서 중앙값 추출
    // 2. 정렬 과정 중 해시맵에 값 저장해서 최빈값 추출

    // 입력 인자 : 중복 허용되는 무작위 정수값으로 이뤄진 문자열

use std::collections::HashMap;
use std::env;
use std::process;


fn main()
{
    let argv:Vec<String> = env::args().collect();
    let mut arr:Vec<i32>;
    let mut mode:HashMap<i32, i32> = HashMap::new();

    let argc = argv.len();

    match argc
    {
        1 => {
            println!("정렬 대상 숫자들을 입력하세요");
            return ()
        },
        2 => arr = parse_argv(argv),
        _ => arr = parse_argv(argv),
    };

    arr.sort();
    let middle_idx = arr.len() / 2;
    println!("중간값 : {}", arr[middle_idx]);

    for i in arr
    {
        let count = mode.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max_key = 0;
    let mut max_val = 0;
    for (key, value) in mode
    {
        if max_val < value
        {
            max_val = value;
            max_key = key;
        }
    }

    println!("최빈값 : {max_key}, 횟수 : {max_val}");
}

fn parse_argv(av:Vec<String>) -> Vec<i32>
{
    let mut result = Vec::new();

    for arg in &av[1..]{
        for part in arg.split_whitespace(){
            match part.parse::<i32>(){
                Ok(num) => result.push(num),
                Err(_) => {
                    println!("{part}는 숫자가 아님");
                    process::exit(1);
                },
            }
        }
    }
    result
}

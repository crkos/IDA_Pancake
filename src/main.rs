use std::cmp::min;
use std::collections::{HashSet};
use rand::prelude::IteratorRandom;

// Hace la permutacion de n pancakes
fn flip_pancakes(pancakes: &mut [char], index: usize) {
    if pancakes.len() < 2 { return; }
    pancakes[..index].reverse();
}

// Genera las permutaciones que siguen del estado
fn generar_sucesores(pancakes: &Vec<char>) -> Vec<Vec<char>> {
    let mut sucesores = Vec::new();
    for i in 1..=pancakes.len() {
        let mut sucesor = pancakes.clone();
        flip_pancakes(&mut sucesor, i);
        sucesores.push(sucesor);
    }
    sucesores
}

// Llena un vector de un diccionario al azar
fn fill_pancakes(num_pancakes: usize) -> Vec<char> {
    let dict = "abcdefghijklmnopqrstuvwxyz";
    let mut empty_pancakes = Vec::new();
    let mut seen_chars = HashSet::new();
    for _ in 0..num_pancakes {
        let mut random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        while seen_chars.contains(&random_char) {
            random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        }
        seen_chars.insert(random_char);
        empty_pancakes.push(random_char);
    }
    empty_pancakes
}

// h4
// Función que implementa la heuristica
fn h4(pancakes: &Vec<char>, target: &Vec<char>) -> i32 {
    let mut c = 0;
    for i in 0..pancakes.len() {
        if pancakes[i] != target[i] {
            c += 1;
        }
        if i < pancakes.len()-1 && (pancakes[i] as i32 - pancakes[i+1] as i32).abs() > 1 {
            c += 1;
        }
    }
    c
}

//Funcion que checa si la permutación esta ordenada
fn is_pancake_sorted(pancakes: &Vec<char>) -> bool {
    for i in 1..pancakes.len() {
        if pancakes[i] < pancakes[i - 1] {
            return false;
        }
    }
    true
}

fn ida_star_recursive(
    curr_pancakes: &Vec<char>,
    target: &Vec<char>,
    g: i32,
    umbral: i32,
    count: &mut i32,
) -> i32 {
    *count += 1;
    let f = g + h4(&curr_pancakes, &target) as i32;
    if f > umbral {
        return f;
    }
    if curr_pancakes == target {
        println!("Solucion encontrada: {:?}", curr_pancakes);
        println!("Numero de nodos visitados: {}", *count);
        return 0;
    }
    let mut proximo_umbral = i32::MAX;
    let sucesores = generar_sucesores(&curr_pancakes);
    for sucesor in sucesores {
        let sucesor_g = g + 1;
        let sucesor_umbral = ida_star_recursive(&sucesor, &target, sucesor_g, umbral, count);
        if sucesor_umbral == 0 {
            return 0;
        }
        proximo_umbral = min(proximo_umbral, sucesor_umbral);
    }
    proximo_umbral
}

fn ida_star(pancakes: &Vec<char>) {
    let target = {
        let mut sorted_pancakes = pancakes.clone();
        sorted_pancakes.sort();
        sorted_pancakes
    };
    let mut umbral = h4(&pancakes, &target) as i32;
    let mut count = 0;
    loop {
        let sucesor_umbral = ida_star_recursive(&pancakes, &target, 0, umbral, &mut count);
        if sucesor_umbral == 0 {
            return;
        }
        umbral = sucesor_umbral;
        println!("Umbral actualizado a: {}", umbral);
    }
}

fn main() {
    let mut n = String::new();
    println!("Ingrese el numero de caracteres de pancakes: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let pancakes = fill_pancakes(n);
    println!("Pancakes generados: {:?}", pancakes);
    ida_star(&pancakes);
}

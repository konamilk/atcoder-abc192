use proconio::input;
// use proconio::marker::Chars;
use std::cmp::Reverse;

fn main() {
    input! {
        mut n: i32,
        k: i32
    }

    // let ans = g2(n);

    for i in 0..k {
        n = f(n)
    }

    println!("{}", n)
}

fn g1(mut x: i32) -> i32 {
    let mut bunkai = vec![];

    while x > 0 {
        bunkai.push(x % 10);
        x = x / 10
    }

    bunkai.sort();

    let mut result = 0;

    for (i,y) in bunkai.into_iter().enumerate() {
        result += 10i32.pow(i as u32) * y;
    }

    result
}

fn g2(mut x: i32) -> i32 {
    let mut bunkai = vec![];

    while x > 0 {
        bunkai.push(x % 10);
        x = x / 10
    }

    bunkai.sort_by_key(|&x| Reverse(x));;

    let mut result = 0;

    for (i,y) in bunkai.into_iter().enumerate() {
        result += 10i32.pow(i as u32) * y;
    }

    result
}

fn f(x: i32) -> i32 {
    g1(x) - g2(x)
}
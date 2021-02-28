use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        x: i32
    }

    let ans = 100 - (x % 100);

    println!("{}",ans)
}

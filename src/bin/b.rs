use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input!{
        strs: Bytes
    }

    let mut ans = "Yes";

    for (i, s) in strs.into_iter().enumerate() {
        if i % 2 == 0 {
            if s < 97 {
                ans = "No";
                break
            }
        }
        else {
            if s >= 97 {
                ans = "No";
                break
            }
        }
    }

    println!("{}", ans)
}

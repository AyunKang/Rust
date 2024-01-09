use std::io;

// 메인 함수
fn main() {
    println!("피보나치 수를 생성하는 프로그램입니다.");

    let mut n = String::new();

    println!("몇번째 수를 구할까요? ");
    io::stdin()
        .read_line(&mut n)
        .expect("숫자를 입력하지 않아 프로그램이 종료됩니다.");

    let n: i32 = n
        .trim()
        .parse()
        .expect("숫자가 범위를 벗어나 프로그램이 종료됩니다.");

    println!("{n}번째 피보나치 수는 {}입니다.", fib(n));
}

// n번째 피보나치 수를 구하는 함수
fn fib(x: i32) -> i32 {
    if x <= 2 {
        return 1;
    } else {
        return fib(x - 1) + fib(x - 2);
    }
}

use std::io;

// 메인 함수
fn main() {
    println!("섭씨, 화씨 온도를 변환해줍니다.");

    let mut mode = String::new();

    println!("어떤 온도를 입력하시나요? (F,C 중 입력) : ");
    io::stdin()
        .read_line(&mut mode)
        .expect("F 또는 C를 입력해주세요.");

    let mode: char = mode.trim().parse().expect("입력값은 F 또는 C여야 합니다.");

    let mut temp = String::new();

    println!("온도를 입력해주세요 : ");
    io::stdin()
        .read_line(&mut temp)
        .expect("숫자를 입력해주세요.");

    let temp: f64 = temp
        .trim()
        .parse()
        .expect("숫자를 입력하지 않아 종료합니다.");

    println!("입력하신 온도는 {temp}입니다.");

    if mode == 'F' {
        println!("변환된 온도는 {}입니다.", f_to_c(temp));
    } else if mode == 'C' {
        println!("변환된 온도는 {}입니다.", c_to_f(temp));
    }
}

// 화씨를 섭씨 온도로 변환해주는 함수
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

// 섭씨를 화씨 온도로 변환해주는 함수
fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

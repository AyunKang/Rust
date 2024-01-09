fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = 56.7 / 32.2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

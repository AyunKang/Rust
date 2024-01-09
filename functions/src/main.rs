fn main() {
    print_labled_measurement(13, 'a');
}

fn print_labled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

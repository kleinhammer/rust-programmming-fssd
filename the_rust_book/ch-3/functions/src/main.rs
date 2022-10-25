fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(55, 'd');
    encapsulated_expression();
}

fn another_function(x: i32) {
    println!("the value of x is {}", { x });
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn encapsulated_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

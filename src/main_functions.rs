fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameters(5,3);
    print_labeled_measurement(5, 'h');
    statements_and_expression();
    let x = five();
    println!("x = {x}");

    let x = plus_one(50);

    println!("The value of x is: {x}");

}

fn plus_one(x: i32) -> i32 {//You can return early from a function by using the return 
    //keyword and specifying a value, but most functions return the last expression implicitly
    x + 1
}

fn five() -> i32 { //the return word is implicity. We don’t name return values, but we must declare their type after an arrow (->)
    5
}

fn statements_and_expression(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) { //In function signatures, you must declare the type of each parameter
    println!("The measurement is: {value}{unit_label}");
}

fn another_function_with_parameters(x: i32, y: i32){
    println!("the sum: {}", (x+y));
}

fn another_function() {
    println!("Another function.");
}

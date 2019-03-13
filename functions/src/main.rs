fn main() {
    println!("Hello, world!");

    another_function(4, 6);
    println!("{}", five());

    let number = 7;
    if number < 5 {
        println!("condition was true", );
    } else {
        println!("condition was false", )
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element)
    }
    for number in 1..4 {
        println!("{}", number);
    }

}

fn another_function(x: i32, y: i32) {
    println!("{}", x);
    println!("{}", y);
}

fn five() -> i32 {
    5
}
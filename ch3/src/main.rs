fn main() {
    let my_temp = f_to_c_temp(72.0);

    println!("The temperature is {}", my_temp);
    
    let n = 8;
    let nth_fib = get_nth_fib(n);

    println!("The {} fibonacci number is {}",n, nth_fib);

    christmas_carol();
}

fn f_to_c_temp(deg_f: f32) -> f32 {
    let ratio: f32 = 5.0/9.0;
    let constant: f32 = 32.0;

    (deg_f - constant) * ratio
}

fn get_nth_fib(n: i32) -> i32 {
    let mut prev_fib = 0;
    let mut current_fib = 1;
    if n < 2 {
        return prev_fib;
    }
    for num in (2..n)
    {
        let temp = current_fib;
        current_fib = current_fib + prev_fib;
        prev_fib = temp;
    }
    return current_fib;
}

fn christmas_carol() {
    let mut i = 1;
    let mut carol_string = String::new();
    let carol_lines = [
        "A partridge in a pear tree", "Two turtle doves",
        "Three french hen", "Four colly birds",
        "Five golden rings", "Six geese a-laying",
        "Seven swans a-swimming"
    ];
    for line in carol_lines.iter()
    {
        let mut current_line = line.to_string();
        if i > 1 { 
            current_line.push(',');
        }
        current_line.push_str("\n");
        current_line.push_str(&carol_string);
        carol_string = current_line;

        println!("\nOn the {} day of Christmas,\nMy true love sent to me\n{}", i, carol_string);
        i+=1;
    }
}

// defining Declrative_macro
macro_rules! my_macro{
    ($x: expr) => {
        println!("the value of x is: {}",$x);
    };
}

// macro to create a loop
macro_rules! my_loop {
    ($start: expr, $end:expr, $body: expr) => {
        for i in $start..$end{
            $body(i) // this is a closure that expects 'i' param.
        }
    };
    
}
fn main() {
    my_macro!("str".to_string());
    my_macro!(123);

    // calling loop macro
    my_loop!(0,10,|i|{
        println!("value of i: {i}");
    });
}

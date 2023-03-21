
/* 'dyn' keyword usage : dyn is used to enable dynamic dispatch,
    which allows calling a method on an object,
    without knowing its exact type at compile time. */
trait MyTrait{
    fn my_method(&self);
}

struct MyStruct();

impl MyTrait for MyStruct{
    fn my_method(&self) {
        println!("Method called for MyStruct");
    }
}

impl MyTrait for String{
    fn my_method(&self) {
        println!("Method called for i32");
    }
}

/// this fn takes 'trait obj' as an arg which implements the 'MyTrait' 
 
fn do_something(trait_obj : &dyn MyTrait){
    trait_obj.my_method();
}

fn main() {
    let my_struct = MyStruct{};
    do_something(&my_struct);

    do_something(&"String".to_string());

    /*  this will be an error
    do_something(&5); */
}

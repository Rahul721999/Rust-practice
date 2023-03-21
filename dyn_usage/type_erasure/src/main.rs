/* 'dyn' keyword also use for 'Type Erasure' which means,
    removing type information on runtime. This is often used in generic
    code to hide impl details 
*/


struct MyStruct<T>{
    obj : Box<dyn SomeTrait<T>>,
}

trait SomeTrait<T>{
    fn do_something(&self, arg : T);
}

struct MyTraitImpl;

impl <T: std::fmt::Debug> SomeTrait<T> for MyTraitImpl{
    fn do_something(&self, arg : T) {
        println!("Arg passed in do_something method is: {:?}", arg);
    }
}
fn main() {
    let my_struct = MyStruct{
        obj : Box::new(MyTraitImpl{}),
    };
    my_struct.obj.do_something("hello"); // gives error untill we declair this line
}

/* In this example 'MyStruct' stores an example that impl the 'SomeTrait' trait,
    but the exact type is hidden behind the struct obj. This allows the impl of 
    'MyStruct' to change without affecting the reast of the code. */
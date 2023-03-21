
/* dyn is used when creating trait objects, 
    which are used to store objects that implement a specific trait*/


trait MyTrait {
    fn my_method(&self);
}
struct MyStruct;

impl MyTrait for MyStruct{
    fn my_method(&self) {
        println!("Method called for MyStruct");
    }
}
fn main() {
    let my_struct = MyStruct{};
    let trait_obj: &dyn MyTrait= &my_struct;
    trait_obj.my_method();

    // MyStruct::my_method(&MyStruct{}); // also works
}

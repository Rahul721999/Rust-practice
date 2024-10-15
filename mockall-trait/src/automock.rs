#![allow(unused)]
use mockall::predicate;
use mockall::*;

// non-clonable struct
pub struct NonClone;

#[automock] // Auto mock macro, helps to mock a trait automatically
pub trait MyTrait1 { // your own custom mock trait
    fn method1(&self, x: f32) -> i32;

    fn method2(&self) -> u32; // return static value
    fn method3(&self, x: u32, y: u32) -> u32; // returns static value

    fn method4(&self) -> NonClone;
}

pub fn test_method1(x: &dyn MyTrait1) -> i32 {
    x.method1(11.11)
}

pub fn automock_test() {
    let mut mock = MockMyTrait1::new(); // creating new Mock version of my trait
    mock.expect_method1()
        .with(predicate::eq(11.11))
        .times(1)
        .returning(|x| x as i32);
    assert_eq!(11, test_method1(&mock));
}

pub fn static_return_test(){
    let mut mock = MockMyTrait1::new();
    mock.expect_method2()
        .return_const(432u32);

    mock.expect_method3()
        .returning(|x, y| x + y);

}

pub fn returning_non_clonable(){
    let mut mock = MockMyTrait1::new();
    let non_cloneable_struct = NonClone;

    mock.expect_method4()
        .return_once(move ||non_cloneable_struct);
}

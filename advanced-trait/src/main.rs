//defining Ractangle struct
#[derive(Debug)]
pub struct Rectangle<T>{
    x :T ,
    y : T,
}



// defining area trait..
pub trait Area{
    type Output;
    fn found_area(&self) -> Option<Self::Output>;
}
impl <T> Area for Rectangle<T>
where T : Copy + std::ops::Mul<Output = T>
{
    type Output = T;
    fn found_area(&self) -> Option<Self::Output> 
    {
        Some(self.x * self.y)
    }
}




// defining new trait with Rhs
pub trait Add<Rhs=Self>{
    type Outp;
    fn add(&self)-> Option<Self::Outp>;
}
// implementing add trait for Rectangle
impl <T> Add for Rectangle<T>
where
    T : Copy + std::ops::Add<Output = T>
{   
    type Outp = T;
    fn add(&self)-> Option<Self::Outp>{
        Some(self.x + self.y)
    }
}


fn main() {
    let rec = Rectangle{
        x : 12, y:12
    };
    if let Some(area) = rec.found_area(){
        println!("the area of the rectangle is : {area}");
    }
    else {
        println!("None value returned");
    }

    if let Some(addition) = rec.add(){
        println!("the addition of height and width is : {addition}");
    }
    else {
        println!("failed to get the addition result");
    }
    let f = Box::new(|| println!("hi"));
    

}


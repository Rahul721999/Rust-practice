use linked_list::utils::*;
fn main() {
    let mut list = LinkedList::<i32>::new();
    list.append(1);
    list.append(2);
    // list.append(3);
    // list.append(4);
    // list.append(5);
    println!("{}",list.remove_last());
    println!("{}",list.display());
    println!("{}", list.count_node());
    
}

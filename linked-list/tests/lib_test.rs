use linked_list::utils::*;

#[test]
fn ll_append_test() {
    let mut ll = LinkedList::new();
    ll.append(1);
    ll.append(2);
    ll.append(3);
    let head = &ll.head.unwrap_or(Box::new(Node::new(0)));
    assert_eq!(head.value, 1);
    assert_eq!(head.next.as_ref().unwrap().value, 2);
    assert_eq!(head.next.as_ref().unwrap().next.as_ref().unwrap().value, 3);
}

#[test]
fn ll_display_test(){
    let mut ll = LinkedList::new();
    ll.append(1);
    ll.append(2);
    ll.append(3);

    // get the output of display fn by calling the helper fn 'get_output'
    let output = ll.display();

    // the actuall output should be..
    let res = "The values of the List are : Head-> 1-> 2-> 3-> ";

    assert_eq!(res, output)
}

// this is a helper fn to capture the output...
/*this fn takes a closure as arg
    // another way to write this fn
    fn get<T : FnOnce() -> ()>(t : &T) -> String{}
 */
// fn get_output(f: &dyn Fn()) -> String{
//     use std::io::{self, Read, Write};
//     let mut output = Vec::new();
//     let mut buffer = std::io::Cursor::new(&mut output);
//     let stdout = std::io::stdout();
//     let mut handle = stdout.lock();
//     std::io::stdout().flush().unwrap();
//     std::io::stdout().write_all(b"\n").unwrap();
//     f();
//     std::io::stdout().flush().unwrap();
//     std::io::stdout().write_all(b"\n").unwrap();
//     handle.read_to_end(&mut buffer).unwrap();
//     let output = String::from_utf8_lossy(&buffer.get_ref()).to_string();
//     output.replace("\r\n", "\n").replace("\r", "\n")

// }
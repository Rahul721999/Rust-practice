#![allow(unused)]

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T>
where
    T: Copy + std::fmt::Display,
{
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}
impl<T> Node<T>
where
    T: Copy + std::fmt::Display,
{
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

// implement copy trait for Node<T>

#[derive(Debug, Clone, PartialEq)]
pub struct LinkedList<T>
where
    T: Copy + std::fmt::Display,
{
    pub head: Option<Box<Node<T>>>,
}
impl<T> LinkedList<T>
where
    T: Copy + std::fmt::Display,
{
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    pub fn append(&mut self, value: T) {
        if let Some(ref mut head) = self.head {
            let mut tail_node = head;
            while let Some(ref mut node) = tail_node.next {
                tail_node = node;
            }
            tail_node.next = Some(Box::new(Node::new(value)));
            println!("Insertion done for value: {value}");
        } else {
            self.head = Some(Box::new(Node::new(value)));
            println!("Insertion done for value: {value}");
        }
    }

    pub fn remove_last(&mut self) -> &str {
        match self.count_node(){
            0 => return "List is empty",
            1 => {
                self.head = None;
                return "Success"
            },
            2 =>{
                self.head.as_mut().expect("Failed to unwrap head value").next = None;
                return "success"
            },
            _ => {
                if let Some(ref mut head) = self.head{
                    let mut head_node = head;
                    // let mut tail_node = unsafe { head_node.next.unwrap_unchecked() }; // I'm too much Over-confident LOL
                    while let Some(ref mut node) = head_node.next{
                        if node.clone().next.expect("failed to get the next node").next.is_none(){
                            node.next = None;
                            return "Success"
                        }else{
                            head_node = node;
                        }
                    }
                    return "Success"
                }else {
                    return "List is empty"
                }
            }
        }
        
    }

    pub fn count_node(&self) -> i32 {
        if let Some(ref head) = self.head{
            let mut head_node = head;
            let mut count = 1;
            while let Some(ref node) = head_node.next{
                head_node = node;
                count += 1 ;
            }
            return count
        }else {
            return 0
        }
    }

    /// fn to display the values of the list...
    pub fn display(&self) -> String {
        if let None = self.head {
            return format!("List is empty");
        };
        let mut res = String::new();
        if let Some(head) = &self.head {
            let mut temp_node = head;
            res.push_str("The values of the List are : Head-> ");
            loop {
                res.push_str(format!("{}-> ", temp_node.value).as_str());
                match &temp_node.next {
                    Some(node) => temp_node = node,
                    None => break,
                }
            }
        }
        return res
    }
    

}

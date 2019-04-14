use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn to_string(&self) -> String { // This may cause stack overflow if your length is too large. However, it won't overflow if you build this program with optimization enable 
        match *self {
            Nil => {
                format!("Nil")
            },
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.to_string())
            }
        }
    }
}

fn main() {
    let mut list = List::new();

    for i in 1..=30000 { 
        list = list.prepend(i);
    }
    
    println!("linked list has length {}", list.len());
    println!("{}", list.to_string());
}

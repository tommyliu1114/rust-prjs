

enum List {
    m_cons(u32,Box<List>),
    Nil,
}

impl List{
    fn new() -> List {
        List::Nil
    }
    fn prepend(self, elem: u32) -> List {
        List::m_cons(elem,Box::new(self))
    }
    fn len(&self) -> u32{
        match *self{
            List::m_cons(_,ref tail ) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
    fn stringfy(&self) -> String {
        match *self {
            List::m_cons(head,ref tail ) => {
                format!("{},{}",head,tail.stringfy())
            },
            List::Nil => format!("Nil"),
        }
    }
}

fn main() {
    let  m_list = List::new();
    let cam_list = m_list.prepend(1).prepend(2).prepend(3);
    println!("linked list has length : {}",cam_list.len());
    println!("{}",cam_list.stringfy());
    println!("Hello, world!");
}

use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    elem: T
}

impl<T> List<T> {

    pub fn new() -> Self {
        List { head: None}
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {head: Some(
            Rc::new(
                Node {next: self.head.clone(),
                elem: elem}
            ))
        }
    }

    pub fn tail(&self) -> List<T> {
        List{head: self.head.as_ref().and_then(|node| node.next.clone())}
    }
    
}
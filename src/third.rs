use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List{ 
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.clone()
        }
    }

}

mod test {
    use super::{List, Node};
    use std::rc::Rc;

    #[test]
    fn test_prepend() {
        let list: List<i32> = List{ head: Some(Rc::new(Node {elem: 5, next: None}))};
        let list2 = list.prepend(6);
        list.prepend(7);

        assert_eq!(list.head.as_ref().unwrap().elem, 5);
        assert_eq!(list.head.as_ref().unwrap().elem, 5);
        // assert_eq!(list.head.as_ref().unwrap().next, );

        // println!("Elem: {}", list.head.as_ref().unwrap().elem);
        // println!("Elem: {}", list.head.as_ref().unwrap().next.as_ref().unwrap().elem);


        // println!("Strong count: {}", Rc::strong_count(&list.head.unwrap()));
                // let list2 = list.prepend(6);
        // let list3 = list.prepend(7);        // assert_eq!(list.head.clone().unwrap().elem, 5);
        // assert_eq!(list.head.clone().unwrap().elem, 5);
        // assert_eq!(list.head.unwrap().elem, 5);

    }
}

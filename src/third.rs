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
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

}

mod test {
    use super::{List, Node};
    use std::rc::Rc;

    #[test]
    fn test_prepend() {
        let mut list: List<i32> = List{ head: Some(Rc::new(Node {elem: 5, next: None}))};
        assert_eq!(list.head.as_ref().unwrap().elem, 5);

        list = list.prepend(6);
        assert_eq!(list.head.as_ref().unwrap().elem, 6);

        list = list.prepend(7);
        assert_eq!(list.head.as_ref().unwrap().elem, 7);

        
        // assert_eq!(list.head.as_ref().unwrap().elem, 5);
        // assert_eq!(list.head.as_ref().unwrap().elem, 5);
        // assert_eq!(list.head.as_ref().unwrap().next, );

        // println!("Elem: {}", list.head.as_ref().unwrap().elem);
        // println!("Elem: {}", list.head.as_ref().unwrap().next.as_ref().unwrap().elem);


        // println!("Strong count: {}", Rc::strong_count(&list.head.unwrap()));
                // let list2 = list.prepend(6);
        // let list3 = list.prepend(7);        // assert_eq!(list.head.clone().unwrap().elem, 5);
        // assert_eq!(list.head.clone().unwrap().elem, 5);
        // assert_eq!(list.head.unwrap().elem, 5);

    }

    #[test]
    fn test_tail() {
        let mut list: List<i32> = List { head: Some(Rc::new(Node {elem: 5, next: None}))};
        list = list.prepend(4);
        list = list.prepend(3);
        list = list.prepend(2);
        let new_list = list.tail();
        assert_eq!(new_list.head.as_ref().unwrap().elem, 3);
        let new_list = list.tail().tail();
        assert_eq!(new_list.head.as_ref().unwrap().elem, 4);
        
        let new_list = list.tail().tail().tail();
        assert_eq!(new_list.head.as_ref().unwrap().elem, 5);
        
        // Will break unless you use map instead of unwrap!
        let new_list = list.tail().tail().tail().tail().tail();
        assert!(new_list.head.is_none());
        // new_list.head.as_ref().unwrap().elem;
        // for i in 1..4 { 
        //     println!("Elem {}", trunc_list)
        // }
        // assert_eq!(list.tail().tail().head.unwrap().elem, 4);


    }
}

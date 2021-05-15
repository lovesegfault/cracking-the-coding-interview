use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new = Rc::new(RefCell::new(Node {
            data,
            next: self.head.take(),
        }));

        self.head = Some(new);
    }

    // pub fn push_back(&mut self, data: T) {
    //     let mut tail = self.head.as_mut();
    //     while !tail.as_ref().and_then(|n| n.next.as_ref()).is_none() {
    //         tail = tail.and_then(|n| n.next.as_mut());
    //     }
    //     let new = Box::new(Node { data, next: None });

    //     if let Some(node) = tail {
    //         node.next = Some(new);
    //     }
    // }

    // pub fn pop_front(&mut self) -> Option<T> {
    //     self.head.take().map(|node| {
    //         self.head = node.next;
    //         node.data
    //     })
    // }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut first = self.head.clone();
        let mut second = self.head.as_mut().and_then(|n| n.borrow().next.clone());

        while second
            .as_ref()
            .and_then(|n| n.borrow().next.clone())
            .is_some()
        {
            first = second;
            second = first.as_mut().and_then(|n| n.borrow().next.clone());
        }

        match second {
            Some(_) => {
                drop(second);
                first.and_then(|the_rc_thing| {
                    (*the_rc_thing)
                        .borrow_mut()
                        .next
                        .take() // at this point, there are only _two_ references to the tail, `second`, above, and `l`.
                        // safe because jesus is watching, our lord and sav-your
                        .map(|l| unsafe {
                            l.replace(std::mem::MaybeUninit::uninit().assume_init())
                        })
                        .map(|n| n.data)
                })
            }
            None => self
                .head
                .take()
                .map(|n| n.replace(unsafe { std::mem::MaybeUninit::uninit().assume_init() }))
                .map(|n| n.data),
        }
    }
}

// impl<T> Drop for LinkedList<T> {
//     fn drop(&mut self) {
//         let mut current = self.head.take();
//         while let Some(mut node) = current {
//             current = node.next.take()
//         }
//     }
// }

fn main() {
    let mut ll: LinkedList<u64> = LinkedList::new();
    for i in 0..100 {
        ll.push_front(i);
    }
    for i in 0..100 {
        println!("pop({}) = {}", i, ll.pop_back().unwrap());
    }
}

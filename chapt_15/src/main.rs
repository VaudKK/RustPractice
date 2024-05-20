#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum ListOverFlow{
    Cons(i32,RefCell<Rc<ListOverFlow>>),
    Nil
}

impl ListOverFlow {
    fn tail(&self) -> Option<&RefCell<Rc<ListOverFlow>>>{
        match self {
            ListOverFlow::Cons(_,item) => Some(item),
            ListOverFlow::Nil => None,
        }
    }
}

struct MyBox<T>(T);

struct CustomSmartPointer{
    data: String
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping pointer with data: `{}`!", self.data);
    }
}

impl <T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    
    }
}

use std::{cell::RefCell, ops::Deref, rc::Weak};

impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}


use crate::List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Rc::new(Cons(1,Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    let b= Cons(4, Rc::clone(&list));
    let c = Cons(5, Rc::clone(&list)); 

    println!("This is what c looks like {:#?}", c);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let c = CustomSmartPointer{
        data: String::from("some stuff")
    };

    println!("Custom pointers will be dropped");


    let a = Rc::new(ListOverFlow::Cons(5, RefCell::new(Rc::new(ListOverFlow::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListOverFlow::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
   // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

}

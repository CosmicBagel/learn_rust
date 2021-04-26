use std::{cell::{Ref, RefCell}, ops::Deref, rc::Rc};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}
use crate::RcList::{RcCons, RcNil};

#[derive(Debug)]
enum RefList {
    RefCons(Rc<RefCell<i32>>, Rc<RefList>),
    RefNil,
}
use crate::RefList::{RefCons, RefNil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    //---------------
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    //---------------
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");
    }
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
    //----------------
    {
        let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = RcCons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = RcCons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    //-----------------
    {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));
        let b = RefCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = RefCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }


}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use smart_pointers::{Rc, RefCell};

#[derive(PartialEq, Clone)]
struct Book {
    title: String,
    author: String,
    year: i16,
}

#[test]
fn test_refcell_and_rc() {
    let book = Book {
        title: "Title1".into(),
        author: "Author1".into(),
        year: 1998,
    };

    let refcell1 = Rc::new(RefCell::new(book.clone()));

    let refcell2 = Rc::clone(&refcell1);
    {
        let borrowed1 = refcell1.borrow().unwrap();
        let borrowed2 = refcell2.borrow().unwrap();

        assert_eq!(*borrowed1.title, book.title);
        assert_eq!(*borrowed2.title, book.title);
        assert!(refcell1.borrow_mut().is_none());
    }
    {
        let mut borrowed_mut = refcell1.borrow_mut().unwrap();
        (*borrowed_mut).year = 2001;
    }

    assert_eq!(refcell2.borrow().unwrap().year, 2001);
}

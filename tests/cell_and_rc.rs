use smart_pointers::{Cell, Rc};

#[test]
fn test_cell_and_rc() {
    let num = Rc::new(Cell::new(5u16));
    let arr1 = [Rc::new(Cell::new(1)), Rc::clone(&num)];
    let arr2 = [Rc::new(Cell::new(2)), Rc::clone(&num)];
    (*num).set(10);

    assert_eq!(10, num.get());
    assert_eq!(10, arr1[1].get());
    assert_eq!(10, arr2[1].get());
}

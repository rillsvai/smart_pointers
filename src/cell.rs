use std::cell::UnsafeCell;

pub struct Cell<T>
where
    T: Copy,
{
    value: UnsafeCell<T>,
}

// should not be implemented!
unsafe impl<T> Sync for Cell<T> where T: Copy {}

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use super::Cell;

    #[test]
    fn test_multithread_set() {
        let num = Arc::new(Cell::new(0));

        let limit = 1000000;
        let expected = limit * 2;

        let x1 = Arc::clone(&num);

        let x2 = Arc::clone(&num);
        let jh1 = thread::spawn(move || {
            for _ in 0..limit {
                x2.set(x2.get() + 1)
            }
        });

        let x3 = Arc::clone(&num);
        let jh2 = thread::spawn(move || {
            for _ in 0..limit {
                let i = x3.get();
                x3.set(i + 1);
            }
        });

        jh1.join().unwrap();
        jh2.join().unwrap();

        assert_eq!(num.get(), x1.get());
        assert_ne!(num.get(), expected);
    }
}

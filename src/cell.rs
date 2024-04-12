use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// should not be implemented! Only for demonstration
unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know that no-one else hold reference on this value so reference invaliding won`t happen
        // SAFETY: we know that concurrent set not allowed because of !Sync (implied via UnsafeCell)
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know that no-one else modify this value because of !Sync (implied via UnsafeCell)
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread};

    use super::Cell;

    #[test]
    fn test_concurrent_set() {
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

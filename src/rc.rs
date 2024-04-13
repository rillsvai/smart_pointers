use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::Cell;

pub struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that only deallocated when last Rc gone
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let refcount = inner.refcount.get();

        if refcount == 1 {
            // SAFETY: only one Rc is left. So we have not references after that and we can free value from memory
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
            return;
        }

        // SAFETY: we have more than one Rc so we do not need drop our value
        inner.refcount.set(refcount - 1);
    }
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });

        Self {
            // SAFETY: we know that into_raw will give non null pointer because we allocated memory above
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

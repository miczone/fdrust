use std::alloc::{alloc, dealloc, Layout};

#[derive(Clone, Debug)]
pub struct HeapList<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> HeapList<T> {
    pub fn new(len: usize) -> Self {
        let ptr = unsafe {
            let layout = Layout::from_size_align_unchecked(len, std::mem::size_of::<T>());
            alloc(layout) as *mut T
        };
        return Self { ptr, len }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len {
            return unsafe { Some(&*(self.ptr.add(idx))) }
        }
        return None
    }

    pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
        if idx < self.len {
            return unsafe { Some(&mut *(self.ptr.add(idx))) }
        }
        return None
    }

    pub fn len(&self) -> usize {
        return self.len
    }
}

impl<T> Drop for HeapList<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.ptr as *mut u8,
                Layout::from_size_align_unchecked(self.len, std::mem::size_of::<T>()),
            )
        };
    }
}

impl<T> std::ops::Index<usize> for HeapList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<usize> for HeapList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(index).unwrap()
    }
}    
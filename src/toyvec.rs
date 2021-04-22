pub struct ToyVec<T> {
    elements: Box<[T]>,
    len: usize,
}


impl<T: Default> ToyVec<T> {
    
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(|| Default::default())
            .take(size) // take size
            .collect::<Vec<_>>() // 
            .into_boxed_slice()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // replace self.elements with new_elements which has twice the capacity,
            // and the self.elements moves to old_elements
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }
    
    // lifetime specifier set `default` lifetime as 'a 
    // to have same lifetime as return value 'a
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // replace last element with default because compiler allows only one &mut
            // so we cannot move the last element owned by ToyVec<T> to return value
            // -> using mem::replace allows to get the last element while replacing it with 
            // default value
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

}




























use crate::ToyVec;



pub struct ToyIter<'vec, T> {
    elements: &'vec [T],
    len: usize,
    pos: usize,
}


impl<'vec, T> ToyIter<'vec, T> {
    
    pub fn from_toyvec(elements: &'vec [T], len: usize) -> Self {
        Self {
            elements,
            len,
            pos: 0,
        }
    }
}

impl<'vec, T> Iterator for ToyIter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

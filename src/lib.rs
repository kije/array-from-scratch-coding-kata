mod base_array;

use base_array::{ArrayClosure, ArrayItem, create_array, ArrayIndex, ArrayCell, Array};
use std::rc::Rc;

pub struct MyAwesomeArray {
    array: Array<ArrayItem>,
    len: usize,
}

impl MyAwesomeArray {
    pub fn new(item: ArrayItem) -> Self {
        Self {
            array: create_array(item, None),
            len: 1,
        }
    }

    pub fn push(&mut self, item: ArrayItem) {
        self.array = create_array(item, Some(self.array.clone()));
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<ArrayItem> {
        let mut arr = self.array.clone();

        let mut iterations_left = self.len - index;

        while iterations_left > 1 {
            iterations_left = iterations_left - 1;
            arr = match arr(ArrayIndex::Rest) {
                ArrayCell::Tail(Some(array1)) => array1,
                _ => return None
            }
        }

        match arr(ArrayIndex::Value) {
            ArrayCell::Head(value) => Some(value),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_array() {
        let array = MyAwesomeArray::new(1);
        assert_eq!(array.get(0).unwrap(), 1);
    }

    #[test]
    fn can_insert_and_read_item() {
        let mut array = MyAwesomeArray::new(1);
        array.push(2);
        array.push(3);
        assert_eq!(array.get(0).unwrap(), 1);
        assert_eq!(array.get(1).unwrap(), 2);
        assert_eq!(array.get(2).unwrap(), 3);
    }
}

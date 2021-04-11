use std::rc::Rc;

pub type ArrayItem = isize;

pub type ArrayClosure<T> = dyn Fn(ArrayIndex) -> ArrayCell<T>;
pub type Array<T> = Rc<Box<ArrayClosure<T>>>;

pub enum ArrayCell<T> {
    Head(T),
    Tail(Option<Array<T>>),
}

pub enum ArrayIndex {
    Value,
    Rest,
}

pub fn create_array(elem: ArrayItem, tail: Option<Array<ArrayItem>>) -> Array<ArrayItem> {
    Rc::new(Box::new(move |index: ArrayIndex| {
        match index {
            ArrayIndex::Value => ArrayCell::Head(elem),
            ArrayIndex::Rest => ArrayCell::Tail(tail.clone()),
        }
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_array() {
        let array = create_array(1, None);
        match array(ArrayIndex::Value) {
            ArrayCell::Head(elem) => assert_eq!(elem, 1),
            _ => panic!(),
        }
    }

    #[test]
    fn can_construct_array_of_length_2() {
        let array1 = create_array(1, None);
        let array2 = create_array(2, Some(array1));

        let elem_from_array_2 = array2(ArrayIndex::Value);
        match elem_from_array_2 {
            ArrayCell::Head(elem) => assert_eq!(elem, 2),
            _ => panic!(),
        }

        let retrieved_array_1 = array2(ArrayIndex::Rest);
        match retrieved_array_1 {
            ArrayCell::Tail(Some(array1)) => {
                match array1(ArrayIndex::Value) {
                    ArrayCell::Head(elem) => assert_eq!(elem, 1),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

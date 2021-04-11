use std::rc::Rc;

pub type ArrayItem = isize;

pub type Array<T> = dyn Fn(ArrayIndex) -> ArrayCell<T>;

pub enum ArrayCell<T> {
    Head(T),
    Tail(Option<Rc<Box<Array<T>>>>),
}

pub enum ArrayIndex {
    ArrayItem,
    Rest,
}

pub fn create_array(elem: ArrayItem, cdr: Option<Rc<Box<Array<ArrayItem>>>>) -> Rc<Box<Array<ArrayItem>>> {
    Rc::new(Box::new(move |index: ArrayIndex| {
        match index {
            ArrayIndex::ArrayItem => ArrayCell::Head(elem),
            ArrayIndex::Rest => ArrayCell::Tail(cdr.clone()),
        }
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_array() {
        let array = create_array(1, None);
        let elem1 = array(ArrayIndex::ArrayItem);
        match elem1 {
            ArrayCell::Head(elem) => assert_eq!(elem, 1),
            _ => panic!(),
        }
    }

    #[test]
    fn can_construct_array_of_length_2() {
        let array1 = create_array(1, None);
        let array2 = create_array(2, Some(array1));

        let elem_from_array_2 = array2(ArrayIndex::ArrayItem);
        match elem_from_array_2 {
            ArrayCell::Head(elem) => assert_eq!(elem, 2),
            _ => panic!(),
        }

        let retrived_array_2 = array2(ArrayIndex::Rest);
        match retrived_array_2 {
            ArrayCell::Tail(Some(array1)) => {
                let elem1 = array1(ArrayIndex::ArrayItem);
                match elem1 {
                    ArrayCell::Head(elem) => assert_eq!(elem, 1),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

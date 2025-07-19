use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};

fn get_mapped_refmut<T, U, F>(cell: &RefCell<T>, mapper: F) -> RefMut<U>
where
    F: FnOnce(&mut T) -> &mut U,
{
    RefMut::map(cell.borrow_mut(), mapper)
}
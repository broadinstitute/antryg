use crate::slate::{Key, Slate};

pub struct ExprTag<'a> {
    slate: &'a Slate,
    key: Key
}
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::expr::Expr;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key {
    uuid: Uuid
}
pub(crate) struct Slate {
    exprs: BTreeMap<Key, Expr>
}
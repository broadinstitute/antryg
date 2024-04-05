use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key {
    uuid: Uuid
}
struct Slate {
    exprs: BTreeMap<Key, Expr>
}
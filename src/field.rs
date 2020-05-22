use std::collections::HashSet;

use attribute::*;
use field_access::*;
use ConstantPoolIndex;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Field {
    pub access_flags: HashSet<FieldAccess>,
    pub name_index: ConstantPoolIndex,
    pub descriptor_index: ConstantPoolIndex,
    pub attributes: Vec<Attribute>,
}

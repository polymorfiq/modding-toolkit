use ue_types::*;

pub trait UFieldLike {
    fn field(&self) -> &UField;
}

impl UFieldLike for UProperty {
    fn field(&self) -> &UField { &self.base_field }
}
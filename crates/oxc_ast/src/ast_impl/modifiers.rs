use crate::ast::{ClassElementModifiers, ClassModifiers};

pub trait ClassModifiersExt {
    fn is_declare(&self) -> bool;
    fn is_abstract(&self) -> bool;
}

pub trait ClassElementModifiersExt {
    fn is_private(&self) -> bool;
    fn is_declare(&self) -> bool;
    fn is_readonly(&self) -> bool;
    fn is_abstract(&self) -> bool;
}

impl ClassModifiersExt for Option<ClassModifiers> {
    #[inline]
    fn is_declare(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.declare)
    }
    #[inline]
    fn is_abstract(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.r#abstract)
    }
}

impl ClassElementModifiersExt for Option<ClassElementModifiers> {
    #[inline]
    fn is_private(&self) -> bool {
        self.and_then(|modifiers| modifiers.accessibility)
            .map_or(false, |accessibility| accessibility.is_private())
    }
    #[inline]
    fn is_declare(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.declare)
    }
    #[inline]
    fn is_readonly(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.readonly)
    }
    #[inline]
    fn is_abstract(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.r#abstract)
    }
}

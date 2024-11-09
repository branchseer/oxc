use crate::ast::{
    ClassElementModifiers, ClassModifiers, FormalParameterModifiers, TSAccessibility,
};

pub trait ClassModifiersExt {
    fn is_declare(&self) -> bool;
    fn is_abstract(&self) -> bool;
}

pub trait ClassElementModifiersExt {
    fn is_private(&self) -> bool;
    fn is_declare(&self) -> bool;
    fn is_readonly(&self) -> bool;
    fn is_abstract(&self) -> bool;
    fn is_static(&self) -> bool;
    fn is_override(&self) -> bool;

    fn accessibility(&self) -> Option<TSAccessibility>;
}

pub trait FormalParameterModifiersExt {
    fn accessibility(&self) -> Option<TSAccessibility>;
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
    #[inline]
    fn is_static(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.r#static)
    }
    #[inline]
    fn is_override(&self) -> bool {
        self.map_or(false, |modifiers| modifiers.r#static)
    }

    #[inline]
    fn accessibility(&self) -> Option<TSAccessibility> {
        self.and_then(|modifiers| modifiers.accessibility)
    }
}

impl FormalParameterModifiersExt for Option<FormalParameterModifiers> {
    #[inline]
    fn accessibility(&self) -> Option<TSAccessibility> {
        self.as_ref().and_then(|modifiers| modifiers.accessibility)
    }
}

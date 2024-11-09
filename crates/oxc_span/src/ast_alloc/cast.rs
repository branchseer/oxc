#[macro_export]
#[doc(hidden)]
macro_rules! assert_explicit {
    ('_) => {
        std::compile_error("The lifetime must be explicit")
    };
    ($l:lifetime) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! cast {
    ($val:expr, $node_type:ident <$lt:lifetime, $from: ty as $to: ty>) => {{
        $crate::assert_explicit!($lt);
        let val: $node_type<$lt, $from> = $val;
        if <$from as $crate::ast_alloc::AstAllocator>::IS_VOID
            == <$to as $crate::ast_alloc::AstAllocator>::IS_VOID
        {
            // https://users.rust-lang.org/t/runtime-specialization-when-types-are-equal/60628/15
            Ok(unsafe {
                ::core::ptr::read(&*::core::mem::ManuallyDrop::new(val)
                    as *const $node_type<$lt, $from>
                    as *const $node_type<$lt, $to>)
            })
        } else {
            Err(val)
        }
    }};
}

#[doc(hidden)]
#[inline]
pub unsafe fn reference_cast<T, U>(reference: &T) -> &U {
    (reference as *const T as *const U).as_ref().unwrap_unchecked()
}

#[macro_export]
#[doc(hidden)]
macro_rules! cast_ref {
    ($val:expr, $node_type:ident <$lt:lifetime, $from: ty as $to: ty>) => {{
        $crate::assert_explicit!($lt);
        if <$from as $crate::ast_alloc::AstAllocator>::IS_VOID
            == <$to as $crate::ast_alloc::AstAllocator>::IS_VOID
        {
            Some(unsafe {
                $crate::ast_alloc::reference_cast::<$node_type<$lt, $from>, $node_type<$lt, $to>>(
                    $val,
                )
            })
        } else {
            None
        }
    }};
}

use crate::ast_alloc::void::VoidAllocator;
pub use cast;
pub use cast_ref;
use oxc_allocator::Allocator;

#[cfg(test)]
#[test]
fn test_cast() {
    use super::*;
    use crate::Span;
    use oxc_allocator::Allocator;

    struct MyAstNode<'a, A: AstAllocator>(Box<'a, Span, A>);
    impl<'a, A: AstAllocator> MyAstNode<'a, A> {
        fn new(allocator: &'a A) -> Self {
            Self(allocator.alloc(Span::new(0, 0)))
        }
    }

    fn convert<'a, A: AstAllocator, B: AstAllocator>(
        node: MyAstNode<'a, A>,
    ) -> Result<MyAstNode<'a, B>, MyAstNode<'a, A>> {
        cast!(node, MyAstNode<'a, A as B>)
    }

    let allocator = Allocator::default();
    let void_allocator = VoidAllocator::default();

    assert!(convert::<_, Allocator>(MyAstNode::new(&allocator)).is_ok());
    assert!(convert::<_, VoidAllocator>(MyAstNode::new(&allocator)).is_err());

    assert!(convert::<_, Allocator>(MyAstNode::new(&void_allocator)).is_err());
    assert!(convert::<_, VoidAllocator>(MyAstNode::new(&void_allocator)).is_ok());
}

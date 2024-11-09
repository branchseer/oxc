mod cast;
mod default;
pub mod traits;
mod void;

pub use cast::*;
pub use traits::AstAllocator;
pub use void::VoidAllocator;

pub type Vec<'a, T, A: AstAllocator = oxc_allocator::Allocator> = A::Vec<'a, T>;
pub type Box<'a, T, A: AstAllocator = oxc_allocator::Allocator> = A::Box<'a, T>;

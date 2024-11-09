mod cast;
mod void;
mod default;
pub mod traits;

pub use cast::*;
pub use void::VoidAllocator;
pub use traits::AstAllocator;

pub type Vec<'a, T, A: AstAllocator = oxc_allocator::Allocator> = A::Vec<'a, T>;
pub type Box<'a, T, A: AstAllocator = oxc_allocator::Allocator> = A::Box<'a, T>;

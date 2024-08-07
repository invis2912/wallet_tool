// Import the `AtomicUsize` type for atomic operations on usize values
use std::sync::atomic::AtomicUsize;

// A static atomic counter to keep track of the number of API calls made
pub static API_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// A constant representing the maximum number of API calls allowed
pub const MAX_API_CALLS: usize = 500;

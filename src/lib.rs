//! Look up the joining type for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_joining_type::{get_joining_type, JoiningType};
//!
//! assert_eq!(get_joining_type('A'), JoiningType::NonJoining);
//! ```

pub mod joining_type;
pub mod tables;
pub use joining_type::get_joining_type;
pub use tables::JoiningType;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-joining-type was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (12, 1, 0);

#[cfg(test)]
mod test {
    use super::{get_joining_type, JoiningType};

    #[test]
    fn test_get_joining_type() {
        assert_eq!(get_joining_type('a'), JoiningType::NonJoining);
        assert_eq!(get_joining_type('.'), JoiningType::NonJoining);
        assert_eq!(get_joining_type('カ'), JoiningType::NonJoining);
        assert_eq!(get_joining_type('🦳'), JoiningType::NonJoining);
        assert_eq!(get_joining_type('ھ'), JoiningType::DualJoining);
        assert_eq!(get_joining_type('𐫍'), JoiningType::LeftJoining);
        assert_eq!(get_joining_type('د'), JoiningType::RightJoining);
        assert_eq!(get_joining_type('𞥋'), JoiningType::Transparent);
        assert_eq!(get_joining_type('ـ'), JoiningType::JoinCausing);
    }
}

//! Look up the joining type or joining group for a character.
//!
//! ### Example
//!
//! ```
//! use unicode_joining_type::{get_joining_group, get_joining_type, JoiningGroup, JoiningType};
//!
//! assert_eq!(get_joining_type('A'), JoiningType::NonJoining);
//! assert_eq!(get_joining_group('ھ'), JoiningGroup::KnottedHeh);
//! ```

mod joining_type;
mod joining_type_tables;
pub use joining_type::get_joining_type;
pub use joining_type_tables::JoiningType;

mod joining_group;
mod joining_group_tables;
pub use joining_group::get_joining_group;
pub use joining_group_tables::JoiningGroup;

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-joining-type was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (13, 0, 0);

#[cfg(test)]
mod test {
    use super::{get_joining_group, get_joining_type, JoiningGroup, JoiningType};

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

    #[test]
    fn test_get_joining_group() {
        assert_eq!(get_joining_group('a'), JoiningGroup::NoJoiningGroup);
        assert_eq!(get_joining_group('.'), JoiningGroup::NoJoiningGroup);
        assert_eq!(get_joining_group('カ'), JoiningGroup::NoJoiningGroup);
        assert_eq!(get_joining_group('🦳'), JoiningGroup::NoJoiningGroup);
        assert_eq!(get_joining_group('ھ'), JoiningGroup::KnottedHeh);
        assert_eq!(get_joining_group('𐫍'), JoiningGroup::ManichaeanHeth);
        assert_eq!(get_joining_group('د'), JoiningGroup::Dal);
        assert_eq!(get_joining_group('𞥋'), JoiningGroup::NoJoiningGroup);
        assert_eq!(get_joining_group('ـ'), JoiningGroup::NoJoiningGroup);
    }
}

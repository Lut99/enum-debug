//  LIB.rs
//    by Lut99
// 
//  Created:
//    10 Dec 2022, 11:37:39
//  Last edited:
//    10 Dec 2022, 14:56:30
//  Auto updated?
//    Yes
// 
//  Description:
//!   A simple crate that adds the `EnumDebug` trait, which allows one to
//!   easily get the variant names of an enum.
// 

use std::fmt::{Debug, Display, Formatter, Result as FResult};

#[cfg(feature = "derive")]
pub use enum_debug_derive::EnumDebug;


/***** PRELUDE *****/
/// Can be used to bring this library's prelude into scope.
pub mod prelude {
    pub use super::EnumDebug;
}





/***** AUXILLARY *****/
/// Implements a formatter that can write the variant name of an enum.
/// 
/// The `Debug` formatter writes the enum name and its variant as given by the `EnumDebug` trait, like `<name>::<variant>`.
/// 
/// The `Display` formatter just writes its name.
/// 
/// You should only use this struct through `variant()` in the `EnumDebug` trait.
/// 
/// # Examples
/// ```rust
/// use enum_debug::EnumDebug;
/// 
/// enum Jedi {
///     ObiWanKenobi,
///     AnakinSkywalker,
///     MaceWindu,
///     MasterYoda,
/// }
/// impl EnumDebug for Jedi {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         use Jedi::*;
///         match self {
///             ObiWanKenobi    => write!(f, "ObiWanKenobi"),
///             AnakinSkywalker => write!(f, "AnakinSkywalker"),
///             MaceWindu       => write!(f, "MaceWindu"),
///             MasterYoda      => write!(f, "MasterYoda"),
///         }
///     }
/// }
/// 
/// assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
/// assert_eq!(format!("{}", Jedi::AnakinSkywalker.variant()), "AnakinSkywalker");
/// assert_eq!(Jedi::MaceWindu.variant().to_string(), "MaceWindu");
/// ```
pub struct EnumDebugFormatter<'this, T: ?Sized> {
    /// The enum to format.
    this : &'this T,
}

impl<'this, T: EnumDebug> Debug for EnumDebugFormatter<'this, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.this.fmt_type_name(f)?;
        write!(f, "::")?;
        self.this.fmt(f)?;
        Ok(())
    }
}
impl<'this, T: EnumDebug> Display for EnumDebugFormatter<'this, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.this.fmt(f)
    }
}





/***** LIBRARY *****/
/// Enums that implement this trait can write their variant name using `variant()`.
/// 
/// # Examples
/// Custom implementation of the trait:
/// ```rust
/// use enum_debug::EnumDebug;
/// 
/// enum Jedi {
///     ObiWanKenobi,
///     AnakinSkywalker,
///     MaceWindu,
///     MasterYoda,
/// }
/// impl EnumDebug for Jedi {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         use Jedi::*;
///         match self {
///             ObiWanKenobi    => write!(f, "ObiWanKenobi"),
///             AnakinSkywalker => write!(f, "AnakinSkywalker"),
///             MaceWindu       => write!(f, "MaceWindu"),
///             MasterYoda      => write!(f, "MasterYoda"),
///         }
///     }
/// }
/// 
/// assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
/// assert_eq!(format!("{}", Jedi::AnakinSkywalker.variant()), "AnakinSkywalker");
/// assert_eq!(Jedi::MaceWindu.variant().to_string(), "MaceWindu");
/// ```
/// 
/// Using derive (enable the `derive` feature):
/// ```rust
/// use enum_debug::EnumDebug;
/// 
/// #[derive(EnumDebug)]
/// enum Jedi {
///     ObiWanKenobi,
///     AnakinSkywalker,
///     MaceWindu,
///     MasterYoda,
/// }
/// 
/// assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
/// assert_eq!(format!("{}", Jedi::AnakinSkywalker.variant()), "AnakinSkywalker");
/// assert_eq!(Jedi::MaceWindu.variant().to_string(), "MaceWindu");
/// ```
/// 
/// You can also specify a custom name:
/// ```rust
/// use enum_debug::EnumDebug;
/// 
/// #[derive(EnumDebug)]
/// #[enum_debug(name = "jedi")]
/// enum Jedi {
///     ObiWanKenobi,
///     AnakinSkywalker,
///     MaceWindu,
///     MasterYoda,
/// }
/// 
/// assert_eq!(format!("{:?}", Jedi::ObiWanKenobi.variant()), "jedi::ObiWanKenobi");
/// ```
pub trait EnumDebug {
    // Override mandatory
    /// Formats this enum as its variant name.
    /// 
    /// Note that this is the only function you have to implement when manually implementing this trait.
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function may error if it failed to write to the given formatter somehow.
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult;



    // Default implementations
    /// Formats this enum so that it writes the name of it.
    /// 
    /// By default, will write its type name (using `std::any::type_name`).
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function may error if it failed to write to the given formatter somehow.
    fn fmt_type_name(&self, f: &mut Formatter<'_>) -> FResult {
        write!(f, "{}", std::any::type_name::<Self>())
    }

    /// Returns a formatter for this enum that writes its variant name.
    /// 
    /// The formatter implements both `Debug` and `Display`. In the former, it also includes the name of the type itself as given by `Self::fmt_type()`.
    /// 
    /// # Returns
    /// A new instance of an EnumDebugFormatter that implements `Debug` and `Display`.
    /// 
    /// # Examples
    /// ```rust
    /// use enum_debug::EnumDebug;
    /// 
    /// #[derive(EnumDebug)]
    /// #[enum_debug(name)]
    /// enum Jedi {
    ///     ObiWanKenobi,
    ///     AnakinSkywalker,
    ///     MaceWindu,
    ///     MasterYoda,
    /// }
    /// 
    /// assert_eq!(format!("{}", Jedi::ObiWanKenobi.variant()), "ObiWanKenobi");
    /// assert_eq!(format!("{:?}", Jedi::AnakinSkywalker.variant()), "Jedi::AnakinSkywalker");
    /// assert_eq!(Jedi::MaceWindu.variant().to_string(), "MaceWindu");
    /// ```
    #[inline]
    fn variant(&self) -> EnumDebugFormatter<'_, Self> {
        EnumDebugFormatter {
            this : self,
        }
    }
}

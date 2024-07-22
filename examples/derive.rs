//  DERIVE.rs
//    by Lut99
//
//  Created:
//    22 Jul 2024, 22:33:34
//  Last edited:
//    23 Jul 2024, 00:04:17
//  Auto updated?
//    Yes
//
//  Description:
//!   Shows some derive examples for the `EnumDebug`-macro.
//

use enum_debug::EnumDebug;


/***** EXAMPLES *****/
#[derive(EnumDebug)]
pub enum Empty {}

#[derive(EnumDebug)]
pub enum One {
    VariantWithoutValue,
}

#[derive(EnumDebug)]
pub enum More {
    VariantWithoutValue,
    VariantWithValue(String),
    VariantWithStruct { field: String },
}

#[derive(EnumDebug)]
#[enum_debug(name = "Foo")]
pub enum Modded1 {
    VariantWithoutValue,
}
#[derive(EnumDebug)]
#[enum_debug(path)]
pub enum Modded2 {
    VariantWithoutValue,
}


fn main() {
    assert_eq!(&format!("{}", One::VariantWithoutValue.variant()), "VariantWithoutValue");
    assert_eq!(&format!("{:?}", One::VariantWithoutValue.variant()), "One::VariantWithoutValue");

    assert_eq!(&format!("{}", More::VariantWithoutValue.variant()), "VariantWithoutValue");
    assert_eq!(&format!("{:?}", More::VariantWithoutValue.variant()), "More::VariantWithoutValue");
    assert_eq!(&format!("{}", More::VariantWithValue("foo".into()).variant()), "VariantWithValue");
    assert_eq!(&format!("{:?}", More::VariantWithValue("foo".into()).variant()), "More::VariantWithValue");
    assert_eq!(&format!("{}", More::VariantWithStruct { field: "foo".into() }.variant()), "VariantWithStruct");
    assert_eq!(&format!("{:?}", More::VariantWithStruct { field: "foo".into() }.variant()), "More::VariantWithStruct");

    assert_eq!(&format!("{}", Modded1::VariantWithoutValue.variant()), "VariantWithoutValue");
    assert_eq!(&format!("{:?}", Modded1::VariantWithoutValue.variant()), "Foo::VariantWithoutValue");
    assert_eq!(&format!("{}", Modded2::VariantWithoutValue.variant()), "VariantWithoutValue");
    assert_eq!(&format!("{:?}", Modded2::VariantWithoutValue.variant()), "derive::Modded2::VariantWithoutValue");
}

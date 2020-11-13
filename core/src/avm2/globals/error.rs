//! `Math` impl

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::method::Method;
use crate::avm2::names::{Multiname, Namespace, QName};
use crate::avm2::object::Object;
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use gc_arena::{GcCell, MutationContext};

// macro_rules! error {
//     ($name:literal, $id:literal) => pub const $name: uint = $id;
// }

pub struct ErrorDef<'a> {
    pub id: i32,
    pub name: &'a str,
    pub message: &'a str,
}

const ERROR_1000: ErrorDef = ErrorDef {
    id: 1000,
    name: "Error",
    message: "The system is out of memory.",
};

const ERROR_1001: ErrorDef = ErrorDef {
    id: 1069,
    name: "ReferenceError",
    message: "Property {} not found for {} and there is no default value.",
};

// macro_rules! math_constants {
//     ($class:ident, $($name:expr => $value:expr),*) => {{
//         $(
//             $class.define_class_trait(Trait::from_const(
//                 QName::new(Namespace::public_namespace(), $name),
//                 Multiname::from(QName::new(Namespace::public_namespace(), "Number")),
//                 Some($value.into()),
//             ));
//         )*
//     }};
// }

// macro_rules! math_method {
//     ($class:ident, $($name:expr => $f:expr),*) => {{
//         $(
//             $class.define_class_trait(Trait::from_method(
//                 QName::new(Namespace::public_namespace(), $name),
//                 Method::from_builtin($f),
//             ));
//         )*
//     }};
// }

// macro_rules! math_wrap_std {
//     ($class:ident, $($name:expr => $std:expr),*) => {{
//         $(
//             $class.define_class_trait(Trait::from_method(
//                 QName::new(Namespace::public_namespace(), $name),
//                 Method::from_builtin(
//                     |activation, _this, args| -> Result<Value<'gc>, Error> {
//                         if let Some(input) = args.get(0) {
//                             Ok($std(input.coerce_to_number(activation)?).into())
//                         } else {
//                             Ok(std::f64::NAN.into())
//                         }
//                     }
//                 ),
//             ));
//         )*
//     }};
// }

/// Implements `Error`'s instance initializer.
pub fn instance_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    // TODO: Replace with actual error type.
    Err("TypeError: Error #1076: Math is not a constructor.".into())
}

/// Implements `Error`'s class initializer.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

/// Construct `Error`'s class.
pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::public_namespace(), "Math"),
        Some(QName::new(Namespace::public_namespace(), "Object").into()),
        Method::from_builtin(instance_init),
        Method::from_builtin(class_init),
        mc,
    );

    let mut write = class.write(mc);

    class
}

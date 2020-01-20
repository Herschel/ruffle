//! `Number` class impl

use crate::avm1::function::Executable;
use crate::avm1::property::Attribute::*;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::value_object::ValueObject;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, Value};
use crate::context::UpdateContext;
use enumset::EnumSet;
use gc_arena::MutationContext;

/// `Number` constructor/function
pub fn number<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    let value = if let Some(val) = args.get(0) {
        val.as_number(avm, context)?
    } else {
        0.0
    };

    // If called from a constructor, populate `this`.
    if let Some(mut vbox) = this.as_value_object() {
        vbox.replace_value(context.gc_context, value.into());
    }

    // If Number is called as a function, return the value.
    Ok(value.into())
}

pub fn create_number_object<'gc>(
    gc_context: MutationContext<'gc, '_>,
    number_proto: Option<Object<'gc>>,
    fn_proto: Option<Object<'gc>>,
) -> Object<'gc> {
    let number = ScriptObject::function(
        gc_context,
        Executable::Native(number),
        fn_proto,
        number_proto,
    );
    let object = number.as_script_object().unwrap();

    object.define_value(
        gc_context,
        "MAX_VALUE",
        std::f64::MAX.into(),
        DontDelete | ReadOnly | DontEnum,
    );

    object.define_value(
        gc_context,
        "MIN_VALUE",
        std::f64::MIN.into(),
        DontDelete | ReadOnly | DontEnum,
    );

    object.define_value(
        gc_context,
        "NaN",
        std::f64::NAN.into(),
        DontDelete | ReadOnly | DontEnum,
    );

    object.define_value(
        gc_context,
        "NEGATIVE_INFINITY",
        std::f64::NEG_INFINITY.into(),
        DontDelete | ReadOnly | DontEnum,
    );

    object.define_value(
        gc_context,
        "POSITIVE_INFINITY",
        std::f64::INFINITY.into(),
        DontDelete | ReadOnly | DontEnum,
    );

    number
}

/// Creates `Number.prototype`.
pub fn create_proto<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let number_proto = ValueObject::empty_box(gc_context, Some(proto));
    let mut object = number_proto.as_script_object().unwrap();

    object.force_set_function(
        "toString",
        to_string,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );
    object.force_set_function(
        "valueOf",
        value_of,
        gc_context,
        EnumSet::empty(),
        Some(fn_proto),
    );

    number_proto
}

/// The digits used in `toString`.
/// Supports radixes up to base 36.
const DIGITS: &[u8] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',
];

/// The values returned by `NaN.toString(radix)` for each radix from 2 to 36.
/// This table was generated in Flash.
/// Not sure where the heck these values actually come from...!?
const TO_STRING_NANS: &[&str] = &[
    "-/0000000000000000000000000000000",
    "-/.//./..././/0.0./0.",
    "-.000000000000000",
    "-/--,,..-,-,0,-",
    "-++-0-.00++-.",
    "-/0,/-,.///*.",
    "-.0000000000",
    "-+,)())-*).",
    "NaN",
    "-&0...0.(.",
    "-,%%.-0(&(",
    "-.(.%&,&&%",
    "-/*+.$&'-.",
    "-$()\x22**%(",
    "-(0000000",
    "-+- )!+,'",
    "--'.( -\x1F.",
    "-.)$+)\x1F--",
    "-/#%/!'.(",
    "-/,0\x1F.#'.",
    "-\x1E\x1C!+%!.",
    "-\x22%\x22\x1B!'*",
    "-%+  \x22+(",
    "-(\x1D\x1A#\x19\x1C\x19",
    "-*\x18\x1D(\x1E\x18\x18",
    "-+\x22\x1F\x19$\x1C%",
    "-,$\x1B\x1A'( ",
    "--\x1F\x1C)'((",
    "-.\x14%*$\x14(",
    "-.#0'\x12$.",
    "-.000000",
    "-/\x1B\x14\x16\x13\x1B.",
    "-/#(\x0F\x16\x15\x16",
    "-/+\x11..\x12\x19",
    "-\x0D\x1E\x1C0\x0D\x1C",
];

pub fn to_string<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    let this = Value::from(this).as_number(avm, context)?;
    let radix = {
        let radix = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .as_number(avm, context)?;
        if radix >= 2.0 && radix <= 36.0 {
            radix as u32
        } else {
            10
        }
    };

    if radix == 10 {
        // Output number as floating-point decimal.
        Ok(Value::from(this).coerce_to_string(avm, context)?.into())
    } else if this.is_finite() {
        // Output truncated integer in specified base.
        let mut n = crate::avm1::value::f64_to_wrapping_i32(this);

        let is_negative = if n < 0 {
            n = -n;
            true
        } else if n > 0 {
            false
        } else {
            return Ok("0".into());
        };
        let mut n = n as u32;

        let mut digits = Vec::new();
        while n > 0 {
            let digit = n % radix;
            n /= radix;
            digits.push(DIGITS[digit as usize] as char);
        }
        if is_negative {
            digits.push('-');
        }
        let out: String = digits.into_iter().rev().collect();
        Ok(out.into())
    } else {
        // TODO: I have no idea what the actual derivation of this is...
        // Probably something funky with ASCII values.
        Ok(TO_STRING_NANS[radix as usize - 2].into())
    }
}

pub fn value_of<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    if let Some(vbox) = this.as_value_object() {
        return Ok(vbox.unbox().as_number(avm, context)?.into());
    }

    Ok(Value::Undefined.into())
}

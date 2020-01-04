//! AVM1 String object

use crate::avm1::property::Attribute::*;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, UpdateContext, Value};
use gc_arena::MutationContext;

/// Implements `new String`
pub fn constructor<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String: unimplemented");
    Ok(Value::Undefined.into())
}

/// Creates `String.prototype`.
pub fn create_proto<'gc>(
    gc_context: MutationContext<'gc, '_>,
    proto: Object<'gc>,
    fn_proto: Object<'gc>,
) -> Object<'gc> {
    let object = ScriptObject::object(gc_context, Some(proto));

    object.as_script_object().unwrap().force_set_function(
        "charAt",
        char_at,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "charCodeAt",
        char_code_at,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "concat",
        concat,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "from_char_code",
        from_char_code,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "indexOf",
        index_of,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "lastIndexOf",
        last_index_of,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "slice",
        slice,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "split",
        split,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "substr",
        substr,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "substring",
        substring,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "toLowerCase",
        to_lower_case,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.as_script_object().unwrap().force_set_function(
        "toUpperCase",
        to_upper_case,
        gc_context,
        DontDelete | ReadOnly | DontEnum,
        Some(fn_proto),
    );

    object.into()
}

fn char_at<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.charAt: unimplemented");
    Ok(Value::Undefined.into())
}

fn char_code_at<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.charCodeAt: unimplemented");
    Ok(std::f64::NAN.into())
}

fn concat<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.concat: unimplemented");
    Ok(Value::Undefined.into())
}

fn from_char_code<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.fromCharCode: unimplemented");
    Ok(Value::Undefined.into())
}

fn index_of<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.indexOf: unimplemented");
    Ok(Value::Undefined.into())
}

fn last_index_of<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.lastIndexOf: unimplemented");
    Ok(Value::Undefined.into())
}

fn length<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.length: unimplemented");
    Ok(0.into())
}

fn slice<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.slice: unimplemented");
    Ok(Value::Undefined.into())
}

fn split<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.split: unimplemented");
    Ok(Value::Undefined.into())
}

fn substr<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.substr: unimplemented");
    Ok(Value::Undefined.into())
}

fn substring<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.substring: unimplemented");
    Ok(Value::Undefined.into())
}

fn to_lower_case<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.toLowerCase: unimplemented");
    Ok(Value::Undefined.into())
}

fn to_upper_case<'gc>(
    _avm: &mut Avm1<'gc>,
    _context: &mut UpdateContext<'_, 'gc, '_>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    log::error!("String.toUpperCase: unimplemented");
    Ok(Value::Undefined.into())
}

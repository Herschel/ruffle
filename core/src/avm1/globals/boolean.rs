//! `Boolean` class impl

use crate::avm1::function::Executable;
use crate::avm1::return_value::ReturnValue;
use crate::avm1::value_object::ValueObject;
use crate::avm1::{Avm1, Error, Object, ScriptObject, TObject, Value};
use crate::context::UpdateContext;
use enumset::EnumSet;
use gc_arena::MutationContext;

/// `Boolean` constructor/function
pub fn boolean<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    let value = if let Some(val) = args.get(0) {
        Value::Bool(val.as_bool(avm.current_swf_version()))
    } else {
        Value::Bool(false)
    };

    // If called from a constructor, populate `this`.
    if let Some(mut vbox) = this.as_value_object() {
        vbox.replace_value(context.gc_context, value.clone());
    }

    // If called as a function, return the value.
    Ok(value.into())
}

pub fn create_boolean_object<'gc>(
    gc_context: MutationContext<'gc, '_>,
    number_proto: Option<Object<'gc>>,
    fn_proto: Option<Object<'gc>>,
) -> Object<'gc> {
    ScriptObject::function(
        gc_context,
        Executable::Native(boolean),
        fn_proto,
        number_proto,
    )
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

pub fn to_string<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    if let Some(vbox) = this.as_value_object() {
        return Ok(vbox.unbox().as_bool(avm.current_swf_version()).into());
    }

    //TODO: This normally falls back to `[object Object]` or `[type Function]`,
    //implying that `toString` and `valueOf` are inherent object properties and
    //not just methods.
    Ok(Value::Undefined.into())
}

pub fn value_of<'gc>(
    avm: &mut Avm1<'gc>,
    context: &mut UpdateContext<'_, 'gc, '_>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<ReturnValue<'gc>, Error> {
    Ok(Value::Undefined.into())
}

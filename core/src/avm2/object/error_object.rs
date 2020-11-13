//! Error objects for scripts

use crate::avm2::activation::Activation;
use crate::avm2::class::Class;
use crate::avm2::globals::error::ErrorDef;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::script_object::{ScriptObjectClass, ScriptObjectData};
use crate::avm2::object::{Object, ObjectPtr, TObject};
use crate::avm2::scope::Scope;
use crate::avm2::string::AvmString;
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::{impl_avm2_custom_object, impl_avm2_custom_object_properties};
use gc_arena::{Collect, GcCell, MutationContext};

#[derive(Clone, Collect, Debug, Copy)]
#[collect(no_drop)]
pub struct ErrorObject<'gc>(GcCell<'gc, ErrorObjectData<'gc>>);

#[derive(Clone, Collect, Debug)]
#[collect(no_drop)]
pub struct ErrorObjectData<'gc> {
    /// Base script object
    base: ScriptObjectData<'gc>,

    id: i32,

    /// The domain this object holds
    name: AvmString<'gc>,

    message: AvmString<'gc>,
}

impl<'gc> ErrorObject<'gc> {
    pub fn new(
        mc: MutationContext<'gc, '_>,
        base_proto: Option<Object<'gc>>,
        id: i32,
        name: AvmString<'gc>,
        message: AvmString<'gc>,
    ) -> Object<'gc> {
        let base = ScriptObjectData::base_new(base_proto, ScriptObjectClass::NoClass);

        ErrorObject(GcCell::allocate(
            mc,
            ErrorObjectData {
                base,
                id,
                name,
                message,
            },
        ))
        .into()
    }

    pub fn from_error_def(
        mc: MutationContext<'gc, '_>,
        base_proto: Option<Object<'gc>>,
        error_def: ErrorDef<'static>,
    ) -> Object<'gc> {
        let base = ScriptObjectData::base_new(base_proto, ScriptObjectClass::NoClass);

        ErrorObject(GcCell::allocate(
            mc,
            ErrorObjectData {
                base,
                id: error_def.id,
                name: error_def.name.into(),
                message: error_def.message.into(),
            },
        ))
        .into()
    }

    /// Construct a primitive subclass.
    pub fn derive(
        mc: MutationContext<'gc, '_>,
        base_proto: Object<'gc>,
        class: GcCell<'gc, Class<'gc>>,
        scope: Option<GcCell<'gc, Scope<'gc>>>,
    ) -> Result<Object<'gc>, Error> {
        let base = ScriptObjectData::base_new(
            Some(base_proto),
            ScriptObjectClass::InstancePrototype(class, scope),
        );

        Ok(ErrorObject(GcCell::allocate(
            mc,
            ErrorObjectData {
                base,
                name: "".into(),
                message: "".into(),
                id: 0,
            },
        ))
        .into())
    }
}

impl<'gc> TObject<'gc> for ErrorObject<'gc> {
    impl_avm2_custom_object!(base);
    impl_avm2_custom_object_properties!(base);

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error> {
        let this: Object<'gc> = Object::ErrorObject(*self);
        Ok(this.into())
    }

    fn construct(
        &self,
        activation: &mut Activation<'_, 'gc, '_>,
        args: &[Value<'gc>],
    ) -> Result<Object<'gc>, Error> {
        let this: Object<'gc> = Object::ErrorObject(*self);
        let message = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .coerce_to_string(activation)?;
        let id = args
            .get(1)
            .unwrap_or(&Value::Undefined)
            .coerce_to_i32(activation)?;

        Ok(ErrorObject::new(
            activation.context.gc_context,
            Some(this),
            id,
            self.0.read().name,
            message,
        ))
    }

    fn derive(
        &self,
        activation: &mut Activation<'_, 'gc, '_>,
        _class: GcCell<'gc, Class<'gc>>,
        _scope: Option<GcCell<'gc, Scope<'gc>>>,
    ) -> Result<Object<'gc>, Error> {
        let this: Object<'gc> = Object::ErrorObject(*self);
        Ok(ErrorObject::new(
            activation.context.gc_context,
            Some(this),
            0,
            "".into(),
            "".into(),
        ))
    }
}

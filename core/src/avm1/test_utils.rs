use crate::avm1::activation::{Activation, ActivationIdentifier};
use crate::avm1::error::Error;
use crate::avm1::globals::system::SystemProperties;
use crate::avm1::{Avm1, Object, Timers, UpdateContext};
use crate::avm2::Avm2;
use crate::backend::audio::{AudioManager, NullAudioBackend};
use crate::backend::locale::NullLocaleBackend;
use crate::backend::log::NullLogBackend;
use crate::backend::navigator::NullNavigatorBackend;
use crate::backend::render::NullRenderer;
use crate::backend::storage::MemoryStorageBackend;
use crate::backend::ui::NullUiBackend;
use crate::backend::video::NullVideoBackend;
use crate::display_object::{MovieClip, Stage, TDisplayObject};
use crate::focus_tracker::FocusTracker;
use crate::library::Library;
use crate::loader::LoadManager;
use crate::prelude::*;
use crate::tag_utils::{SwfMovie, SwfSlice};
use crate::vminterface::Instantiator;
use gc_arena::{rootless_arena, MutationContext};
use instant::Instant;
use rand::{rngs::SmallRng, SeedableRng};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub fn with_avm<F>(swf_version: u8, test: F)
where
    F: for<'a, 'gc> FnOnce(&mut Activation<'_, 'gc, '_>, Object<'gc>) -> Result<(), Error<'gc>>,
{
    fn in_the_arena<'a, 'gc: 'a, F>(swf_version: u8, test: F, gc_context: MutationContext<'gc, '_>)
    where
        F: FnOnce(&mut Activation<'_, 'gc, '_>, Object<'gc>) -> Result<(), Error<'gc>>,
    {
        let avm1 = Avm1::new(gc_context, swf_version);
        let avm2 = Avm2::new(gc_context);
        let swf = Arc::new(SwfMovie::empty(swf_version));
        let root: DisplayObject<'gc> =
            MovieClip::new(SwfSlice::empty(swf.clone()), gc_context).into();
        root.set_depth(gc_context, 0);
        let stage = Stage::empty(gc_context, 550, 400);
        let frame_rate = 12.0;
        let globals = avm1.global_object_cell();

        let mut context = UpdateContext {
            gc_context,
            player_data: &mut crate::player::PlayerData {
                player_version: 32,
                swf: swf,
                rng: SmallRng::from_seed([0u8; 32]),
                audio: Box::new(NullAudioBackend::new()),
                ui: Box::new(NullUiBackend::new()),
                navigator: Box::new(NullNavigatorBackend::new()),
                renderer: Box::new(NullRenderer::new()),
                storage: Box::new(MemoryStorageBackend::default()),
                locale: Box::new(NullLocaleBackend::new()),
                log: Box::new(NullLogBackend::new()),
                video: Box::new(NullVideoBackend::new()),
                mouse_pos: (Twips::zero(), Twips::zero()),
                system: SystemProperties::default(),
                warn_on_unsupported_content: false,
                is_playing: true,
                needs_render: false,
                transform_stack: crate::transform::TransformStack::new(),
                frame_rate: frame_rate,
                frame_accumulator: 0.0,
                recent_run_frame_timings: Default::default(),
                instance_counter: 0,
                max_execution_duration: Duration::from_secs(15),
                update_start: Instant::now(),
                time_offset: 0,
                is_mouse_down: false,
                mouse_cursor: crate::backend::ui::MouseCursor::Arrow,
                self_reference: None,
                current_frame: None,
                times_get_time_called: 0,
                time_til_next_timer: None,
            },
            gc_data: &mut crate::player::GcRootData {
                stage,
                action_queue: crate::context::ActionQueue::new(),
                audio_manager: AudioManager::new(),
                library: Library::empty(gc_context),
                mouse_hovered_object: None,
                drag_object: None,
                load_manager: LoadManager::new(),
                shared_objects: HashMap::new(),
                unbound_text_fields: Vec::new(),
                timers: Timers::new(),
                current_context_menu: None,
                avm1,
                avm2,
                external_interface: Default::default(),
                focus_tracker: FocusTracker::new(gc_context),
            },
        };
        context
            .gc_data
            .stage
            .replace_at_depth(&mut context, root, 0);

        root.post_instantiation(&mut context, root, None, Instantiator::Movie, false);
        root.set_name(context.gc_context, "");

        fn run_test<'a, 'gc: 'a, F>(
            activation: &mut Activation<'_, 'gc, '_>,
            root: DisplayObject<'gc>,
            test: F,
        ) where
            F: FnOnce(&mut Activation<'_, 'gc, '_>, Object<'gc>) -> Result<(), Error<'gc>>,
        {
            let this = root.object().coerce_to_object(activation);
            let result = test(activation, this);
            if let Err(e) = result {
                panic!("Encountered exception during test: {}", e);
            }
        }

        let swf_version = context.player_data.swf.version();
        let mut activation = Activation::from_nothing(
            context,
            ActivationIdentifier::root("[Test]"),
            swf_version,
            globals,
            root,
        );

        run_test(&mut activation, root, test)
    }

    rootless_arena(|gc_context| in_the_arena(swf_version, test, gc_context))
}

macro_rules! test_method {
    ( $test: ident, $name: expr, $object: expr, $($versions: expr => { $([$($arg: expr),*] => $out: expr),* }),* ) => {
        #[test]
        fn $test() {
            use $crate::avm1::test_utils::*;
            $(
                for version in &$versions {
                    with_avm(*version, |activation, _root| -> Result<(), Error> {
                        let object = $object(activation);
                        let function = object.get($name, activation)?;

                        $(
                            let args: Vec<Value> = vec![$($arg.into()),*];
                            assert_eq!(function.call($name, activation, object, None, &args)?, $out.into(), "{:?} => {:?} in swf {}", args, $out, version);
                        )*

                        Ok(())
                    });
                }
            )*
        }
    };
}

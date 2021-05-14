//! Contexts and helper types passed between functions.

use crate::avm1::globals::system::SystemProperties;
use crate::avm1::{Avm1, Object as Avm1Object, Timers, Value as Avm1Value};
use crate::avm2::{Avm2, Object as Avm2Object, Value as Avm2Value};
use crate::backend::{
    audio::{AudioBackend, AudioManager, SoundHandle, SoundInstanceHandle},
    locale::LocaleBackend,
    log::LogBackend,
    navigator::NavigatorBackend,
    render::RenderBackend,
    storage::StorageBackend,
    ui::UiBackend,
    video::VideoBackend,
};
use crate::context_menu::ContextMenuState;
use crate::display_object::{EditText, MovieClip, SoundTransform, Stage};
use crate::external::ExternalInterface;
use crate::focus_tracker::FocusTracker;
use crate::library::Library;
use crate::loader::LoadManager;
use crate::player::{GcRootData, Player, PlayerData};
use crate::prelude::*;
use crate::tag_utils::{SwfMovie, SwfSlice};
use crate::transform::TransformStack;
use core::fmt;
use gc_arena::{Collect, MutationContext};
use instant::Instant;
use rand::rngs::SmallRng;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

/// `UpdateContext` holds shared data that is used by the various subsystems of Ruffle.
/// `Player` crates this when it begins a tick and passes it through the call stack to
/// children and the VM.
pub struct UpdateContext<'a, 'gc, 'gc_context> {
    player_data: &'a mut PlayerData,
    gc_data: &'a mut GcRootData<'gc>,
    /// The mutation context to allocate and mutate `GcCell` types.
    pub gc_context: MutationContext<'gc, 'gc_context>,
}

/// Convenience methods for controlling audio.
impl<'a, 'gc, 'gc_context> UpdateContext<'a, 'gc, 'gc_context> {
    pub fn update_sounds(&mut self) {
        self.gc_data.audio_manager.update_sounds(
            self.player_data.audio,
            self.gc_context,
            self.gc_data.action_queue,
            self.gc_data.stage.root_clip(),
        );
    }

    pub fn global_sound_transform(&self) -> &SoundTransform {
        self.gc_data.audio_manager.global_sound_transform()
    }

    pub fn set_global_sound_transform(&mut self, sound_transform: SoundTransform) {
        self.gc_data
            .audio_manager
            .set_global_sound_transform(sound_transform);
    }

    pub fn start_sound(
        &mut self,
        sound: SoundHandle,
        settings: &swf::SoundInfo,
        owner: Option<DisplayObject<'gc>>,
        avm1_object: Option<crate::avm1::SoundObject<'gc>>,
    ) -> Option<SoundInstanceHandle> {
        self.gc_data
            .audio_manager
            .start_sound(self.audio, sound, settings, owner, avm1_object)
    }

    pub fn stop_sound(&mut self, instance: SoundInstanceHandle) {
        self.gc_data.audio_manager.stop_sound(self.audio, instance)
    }

    pub fn stop_sounds_with_handle(&mut self, sound: SoundHandle) {
        self.gc_data
            .audio_manager
            .stop_sounds_with_handle(self.audio, sound)
    }

    pub fn stop_sounds_with_display_object(&mut self, display_object: DisplayObject<'gc>) {
        self.gc_data
            .audio_manager
            .stop_sounds_with_display_object(self.audio, display_object)
    }

    pub fn stop_all_sounds(&mut self) {
        self.gc_data.audio_manager.stop_all_sounds(self.audio)
    }

    pub fn is_sound_playing_with_handle(&mut self, sound: SoundHandle) -> bool {
        self.gc_data
            .audio_manager
            .is_sound_playing_with_handle(sound)
    }

    pub fn start_stream(
        &mut self,
        stream_handle: Option<SoundHandle>,
        movie_clip: MovieClip<'gc>,
        frame: u16,
        data: crate::tag_utils::SwfSlice,
        stream_info: &swf::SoundStreamHead,
    ) -> Option<SoundInstanceHandle> {
        self.gc_data.audio_manager.start_stream(
            self.player_data.audio,
            stream_handle,
            movie_clip,
            frame,
            data,
            stream_info,
        )
    }

    pub fn set_sound_transforms_dirty(&mut self) {
        self.gc_data.audio_manager.set_sound_transforms_dirty()
    }
}

impl<'a, 'gc, 'gc_context> UpdateContext<'a, 'gc, 'gc_context> {
    /// Transform a borrowed update context into an owned update context with
    /// a shorter internal lifetime.
    ///
    /// This is particularly useful for structures that may wish to hold an
    /// update context without adding further lifetimes for its borrowing.
    /// Please note that you will not be able to use the original update
    /// context until this reborrowed copy has fallen out of scope.
    pub fn reborrow<'b>(&'b mut self) -> UpdateContext<'b, 'gc, 'gc_context>
    where
        'a: 'b,
    {
        UpdateContext {
            player_data: self.player_data,
            gc_data: self.gc_data,
            gc_context: self.gc_context,
        }
    }
}

/// A queued ActionScript call.
#[derive(Collect)]
#[collect(no_drop)]
pub struct QueuedActions<'gc> {
    /// The movie clip this ActionScript is running on.
    pub clip: DisplayObject<'gc>,

    /// The type of action this is, along with the corresponding bytecode/method data.
    pub action_type: ActionType<'gc>,

    /// Whether this is an unload action, which can still run if the clip is removed.
    pub is_unload: bool,
}

/// Action and gotos need to be queued up to execute at the end of the frame.
#[derive(Collect)]
#[collect(no_drop)]
pub struct ActionQueue<'gc> {
    /// Each priority is kept in a separate bucket.
    action_queue: Vec<VecDeque<QueuedActions<'gc>>>,
}

impl<'gc> ActionQueue<'gc> {
    const DEFAULT_CAPACITY: usize = 32;
    const NUM_PRIORITIES: usize = 3;

    /// Crates a new `ActionQueue` with an empty queue.
    pub fn new() -> Self {
        let mut action_queue = Vec::with_capacity(Self::NUM_PRIORITIES);
        for _ in 0..Self::NUM_PRIORITIES {
            action_queue.push(VecDeque::with_capacity(Self::DEFAULT_CAPACITY))
        }
        Self { action_queue }
    }

    /// Queues ActionScript to run for the given movie clip.
    /// `actions` is the slice of ActionScript bytecode to run.
    /// The actions will be skipped if the clip is removed before the actions run.
    pub fn queue_actions(
        &mut self,
        clip: DisplayObject<'gc>,
        action_type: ActionType<'gc>,
        is_unload: bool,
    ) {
        let priority = action_type.priority();
        let action = QueuedActions {
            clip,
            action_type,
            is_unload,
        };
        debug_assert!(priority < Self::NUM_PRIORITIES);
        if let Some(queue) = self.action_queue.get_mut(priority) {
            queue.push_back(action)
        }
    }

    /// Sorts and drains the actions from the queue.
    pub fn pop_action(&mut self) -> Option<QueuedActions<'gc>> {
        for queue in self.action_queue.iter_mut().rev() {
            let action = queue.pop_front();
            if action.is_some() {
                return action;
            }
        }
        None
    }
}

impl<'gc> Default for ActionQueue<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared data used during rendering.
/// `Player` creates this when it renders a frame and passes it down to display objects.
pub struct RenderContext<'a, 'gc> {
    /// The renderer, used by the display objects to draw themselves.
    pub renderer: &'a mut dyn RenderBackend,

    /// The UI backend, used to detect user interactions.
    pub ui: &'a mut dyn UiBackend,

    /// The library, which provides access to fonts and other definitions when rendering.
    pub library: &'a Library<'gc>,

    /// The transform stack controls the matrix and color transform as we traverse the display hierarchy.
    pub transform_stack: &'a mut TransformStack,

    /// The current player's stage (including all loaded levels)
    pub stage: Stage<'gc>,

    /// The stack of clip depths, used in masking.
    pub clip_depth_stack: Vec<Depth>,

    /// Whether to allow pushing a new mask. A masker-inside-a-masker does not work in Flash, instead
    /// causing the inner mask to be included as part of the outer mask. Maskee-inside-a-maskee works as one expects.
    pub allow_mask: bool,
}

/// The type of action being run.
#[derive(Clone, Collect)]
#[collect(no_drop)]
pub enum ActionType<'gc> {
    /// Normal frame or event actions.
    Normal { bytecode: SwfSlice },

    /// AVM1 initialize clip event
    Initialize { bytecode: SwfSlice },

    /// Construct a movie with a custom class or on(construct) events
    Construct {
        constructor: Option<Avm1Object<'gc>>,
        events: Vec<SwfSlice>,
    },

    /// An event handler method, e.g. `onEnterFrame`.
    Method {
        object: Avm1Object<'gc>,
        name: &'static str,
        args: Vec<Avm1Value<'gc>>,
    },

    /// A system listener method,
    NotifyListeners {
        listener: &'static str,
        method: &'static str,
        args: Vec<Avm1Value<'gc>>,
    },

    /// An AVM2 callable, e.g. a frame script or event handler.
    Callable2 {
        callable: Avm2Object<'gc>,
        reciever: Option<Avm2Object<'gc>>,
        args: Vec<Avm2Value<'gc>>,
    },
}

impl ActionType<'_> {
    fn priority(&self) -> usize {
        match self {
            ActionType::Initialize { .. } => 2,
            ActionType::Construct { .. } => 1,
            _ => 0,
        }
    }
}

impl fmt::Debug for ActionType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActionType::Normal { bytecode } => f
                .debug_struct("ActionType::Normal")
                .field("bytecode", bytecode)
                .finish(),
            ActionType::Initialize { bytecode } => f
                .debug_struct("ActionType::Initialize")
                .field("bytecode", bytecode)
                .finish(),
            ActionType::Construct {
                constructor,
                events,
            } => f
                .debug_struct("ActionType::Construct")
                .field("constructor", constructor)
                .field("events", events)
                .finish(),
            ActionType::Method { object, name, args } => f
                .debug_struct("ActionType::Method")
                .field("object", object)
                .field("name", name)
                .field("args", args)
                .finish(),
            ActionType::NotifyListeners {
                listener,
                method,
                args,
            } => f
                .debug_struct("ActionType::NotifyListeners")
                .field("listener", listener)
                .field("method", method)
                .field("args", args)
                .finish(),
            ActionType::Callable2 {
                callable,
                reciever,
                args,
            } => f
                .debug_struct("ActionType::Callable2")
                .field("callable", callable)
                .field("reciever", reciever)
                .field("args", args)
                .finish(),
        }
    }
}

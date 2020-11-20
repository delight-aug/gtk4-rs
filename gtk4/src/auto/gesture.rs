// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::EventController;
use crate::EventSequenceState;
use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Gesture(Object<ffi::GtkGesture, ffi::GtkGestureClass>) @extends EventController;

    match fn {
        get_type => || ffi::gtk_gesture_get_type(),
    }
}

pub const NONE_GESTURE: Option<&Gesture> = None;

pub trait GestureExt: 'static {
    fn get_bounding_box(&self) -> Option<gdk::Rectangle>;

    fn get_bounding_box_center(&self) -> Option<(f64, f64)>;

    fn get_device(&self) -> Option<gdk::Device>;

    fn get_group(&self) -> Vec<Gesture>;

    fn get_last_event(&self, sequence: Option<&gdk::EventSequence>) -> Option<gdk::Event>;

    fn get_last_updated_sequence(&self) -> Option<gdk::EventSequence>;

    fn get_point(&self, sequence: Option<&gdk::EventSequence>) -> Option<(f64, f64)>;

    fn get_sequence_state(&self, sequence: &gdk::EventSequence) -> EventSequenceState;

    fn get_sequences(&self) -> Vec<gdk::EventSequence>;

    fn group<P: IsA<Gesture>>(&self, gesture: &P);

    fn handles_sequence(&self, sequence: Option<&gdk::EventSequence>) -> bool;

    fn is_active(&self) -> bool;

    fn is_grouped_with<P: IsA<Gesture>>(&self, other: &P) -> bool;

    fn is_recognized(&self) -> bool;

    fn set_sequence_state(&self, sequence: &gdk::EventSequence, state: EventSequenceState) -> bool;

    fn set_state(&self, state: EventSequenceState) -> bool;

    fn ungroup(&self);

    fn get_property_n_points(&self) -> u32;

    fn connect_begin<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_cancel<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_end<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_sequence_state_changed<
        F: Fn(&Self, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_update<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Gesture>> GestureExt for O {
    fn get_bounding_box(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    fn get_bounding_box_center(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_get_bounding_box_center(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn get_device(&self) -> Option<gdk::Device> {
        unsafe { from_glib_none(ffi::gtk_gesture_get_device(self.as_ref().to_glib_none().0)) }
    }

    fn get_group(&self) -> Vec<Gesture> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_group(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_last_event(&self, sequence: Option<&gdk::EventSequence>) -> Option<gdk::Event> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_last_event(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn get_last_updated_sequence(&self) -> Option<gdk::EventSequence> {
        unsafe {
            from_glib_none(ffi::gtk_gesture_get_last_updated_sequence(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_point(&self, sequence: Option<&gdk::EventSequence>) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_get_point(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn get_sequence_state(&self, sequence: &gdk::EventSequence) -> EventSequenceState {
        unsafe {
            from_glib(ffi::gtk_gesture_get_sequence_state(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn get_sequences(&self) -> Vec<gdk::EventSequence> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_gesture_get_sequences(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn group<P: IsA<Gesture>>(&self, gesture: &P) {
        unsafe {
            ffi::gtk_gesture_group(
                self.as_ref().to_glib_none().0,
                gesture.as_ref().to_glib_none().0,
            );
        }
    }

    fn handles_sequence(&self, sequence: Option<&gdk::EventSequence>) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_handles_sequence(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::gtk_gesture_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn is_grouped_with<P: IsA<Gesture>>(&self, other: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_grouped_with(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_recognized(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_is_recognized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_sequence_state(&self, sequence: &gdk::EventSequence, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_sequence_state(
                self.as_ref().to_glib_none().0,
                mut_override(sequence.to_glib_none().0),
                state.to_glib(),
            ))
        }
    }

    fn set_state(&self, state: EventSequenceState) -> bool {
        unsafe {
            from_glib(ffi::gtk_gesture_set_state(
                self.as_ref().to_glib_none().0,
                state.to_glib(),
            ))
        }
    }

    fn ungroup(&self) {
        unsafe {
            ffi::gtk_gesture_ungroup(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_n_points(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"n-points\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `n-points` getter")
                .unwrap()
        }
    }

    fn connect_begin<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn begin_trampoline<P, F: Fn(&P, Option<&gdk::EventSequence>) + 'static>(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Gesture>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_cancel<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<
            P,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Gesture>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn end_trampoline<P, F: Fn(&P, Option<&gdk::EventSequence>) + 'static>(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Gesture>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sequence_state_changed<
        F: Fn(&Self, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn sequence_state_changed_trampoline<
            P,
            F: Fn(&P, Option<&gdk::EventSequence>, EventSequenceState) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            state: ffi::GtkEventSequenceState,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Gesture>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"sequence-state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    sequence_state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_update<F: Fn(&Self, Option<&gdk::EventSequence>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn update_trampoline<
            P,
            F: Fn(&P, Option<&gdk::EventSequence>) + 'static,
        >(
            this: *mut ffi::GtkGesture,
            sequence: *mut gdk::ffi::GdkEventSequence,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Gesture>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Gesture::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gdk::EventSequence>::from_glib_borrow(sequence)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Gesture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gesture")
    }
}

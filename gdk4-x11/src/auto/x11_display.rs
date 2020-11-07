// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use std::mem;
use std::ptr;
use xlib;
use X11Screen;

glib_wrapper! {
    pub struct X11Display(Object<gdk_x11_sys::GdkX11Display, gdk_x11_sys::GdkX11DisplayClass>) @extends gdk::Display;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_display_get_type(),
    }
}

impl X11Display {
    //pub fn broadcast_startup_message(&self, message_type: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gdk_x11_sys:gdk_x11_display_broadcast_startup_message() }
    //}

    pub fn error_trap_pop(&self) -> i32 {
        unsafe { gdk_x11_sys::gdk_x11_display_error_trap_pop(self.to_glib_none().0) }
    }

    pub fn error_trap_pop_ignored(&self) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_error_trap_pop_ignored(self.to_glib_none().0);
        }
    }

    pub fn error_trap_push(&self) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_error_trap_push(self.to_glib_none().0);
        }
    }

    pub fn get_default_group(&self) -> Option<gdk::Surface> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_display_get_default_group(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_glx_version(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_x11_sys::gdk_x11_display_get_glx_version(
                self.to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            let major = major.assume_init();
            let minor = minor.assume_init();
            if ret {
                Some((major, minor))
            } else {
                None
            }
        }
    }

    pub fn get_primary_monitor(&self) -> Option<gdk::Monitor> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_display_get_primary_monitor(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_screen(&self) -> Option<X11Screen> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_display_get_screen(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_startup_notification_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_x11_sys::gdk_x11_display_get_startup_notification_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_user_time(&self) -> u32 {
        unsafe { gdk_x11_sys::gdk_x11_display_get_user_time(self.to_glib_none().0) }
    }

    pub fn get_xcursor(&self, cursor: &gdk::Cursor) -> xlib::Cursor {
        unsafe {
            gdk_x11_sys::gdk_x11_display_get_xcursor(self.to_glib_none().0, cursor.to_glib_none().0)
        }
    }

    pub fn get_xrootwindow(&self) -> xlib::Window {
        unsafe { gdk_x11_sys::gdk_x11_display_get_xrootwindow(self.to_glib_none().0) }
    }

    pub fn grab(&self) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_grab(self.to_glib_none().0);
        }
    }

    pub fn set_cursor_theme(&self, theme: Option<&str>, size: i32) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_set_cursor_theme(
                self.to_glib_none().0,
                theme.to_glib_none().0,
                size,
            );
        }
    }

    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    pub fn set_surface_scale(&self, scale: i32) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_set_surface_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn string_to_compound_text(&self, str: &str) -> (i32, GString, i32, Vec<u8>) {
        unsafe {
            let mut encoding = ptr::null();
            let mut format = mem::MaybeUninit::uninit();
            let mut ctext = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = gdk_x11_sys::gdk_x11_display_string_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                &mut encoding,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            );
            let format = format.assume_init();
            (
                ret,
                from_glib_none(encoding),
                format,
                FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as usize),
            )
        }
    }

    pub fn ungrab(&self) {
        unsafe {
            gdk_x11_sys::gdk_x11_display_ungrab(self.to_glib_none().0);
        }
    }

    pub fn utf8_to_compound_text(&self, str: &str) -> Option<(GString, i32, Vec<u8>)> {
        unsafe {
            let mut encoding = ptr::null();
            let mut format = mem::MaybeUninit::uninit();
            let mut ctext = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_x11_sys::gdk_x11_display_utf8_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                &mut encoding,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            ));
            let format = format.assume_init();
            if ret {
                Some((
                    from_glib_none(encoding),
                    format,
                    FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as usize),
                ))
            } else {
                None
            }
        }
    }

    pub fn open(display_name: Option<&str>) -> Option<gdk::Display> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gdk_x11_sys::gdk_x11_display_open(
                display_name.to_glib_none().0,
            ))
        }
    }

    pub fn set_program_class<P: IsA<gdk::Display>>(display: &P, program_class: &str) {
        assert_initialized_main_thread!();
        unsafe {
            gdk_x11_sys::gdk_x11_display_set_program_class(
                display.as_ref().to_glib_none().0,
                program_class.to_glib_none().0,
            );
        }
    }

    //pub fn connect_xevent<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented xevent: *.Pointer
    //}
}

impl fmt::Display for X11Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Display")
    }
}

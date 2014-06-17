// This file is part of rustgtk.
//
// rustgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rustgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public License
// along with rustgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A button which pops up a scale

use std::ptr;
use libc::{c_double};
use libc::{c_void};

use traits::{GtkWidget, GtkButton, GtkContainer, GtkScaleButton, GtkOrientable, Signal};
use ffi;
use std;
use gtk::enums::GtkIconSize;

/** 
* ScaleButton — A button which pops up a scale
*
* # Availables signals :
* * `popdown` : Action
* * `popup` : Action
* * `value-changed` : Run Last
*/
pub struct ScaleButton {
    pointer:           *ffi::C_GtkWidget,
    can_drop:          bool,
    signal_handlers:   Vec<Box<SignalHandler>>
}

impl ScaleButton {
    // FIXME: icons -> last parameter
    pub fn new(size: GtkIconSize, min: f64, max: f64, step: f64) -> Option<ScaleButton> {
        let tmp_pointer = unsafe { ffi::gtk_scale_button_new(size, min as c_double, max as c_double, step as c_double, ptr::null()) };
        check_pointer!(tmp_pointer, ScaleButton)
    }
}



impl_GtkWidget!(ScaleButton)
redirect_callback!(ScaleButton)
redirect_callback_widget!(ScaleButton)
struct_signal!(ScaleButton)
impl_signals!(ScaleButton)

impl GtkContainer for ScaleButton {}
impl GtkButton for ScaleButton {}
impl GtkScaleButton for ScaleButton {}
impl GtkOrientable for ScaleButton {}

use libc::{c_double};
use std::mem::transmute;
use cairo::enums::{
};
use cairo::types::{
};
use cairo::ffi;
use cairo;

impl cairo::Context{
    pub fn translate(&self, tx: f64, ty: f64){
        unsafe{
            ffi::cairo_translate(self.get_ptr(), tx, ty)
        }
    }

    pub fn scale(&self, sx: f64, sy: f64){
        unsafe{
            ffi::cairo_scale(self.get_ptr(), sx, sy)
        }
    }

    pub fn rotate(&self, angle: f64){
        unsafe{
            ffi::cairo_rotate(self.get_ptr(), angle)
        }
    }

    //pub fn cairo_transform(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_set_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    //pub fn cairo_get_matrix(cr: *cairo_t, matrix: *cairo_matrix_t);

    pub fn identity_matrix(&self){
        unsafe{
            ffi::cairo_identity_matrix(self.get_ptr())
        }
    }

    pub fn user_to_device(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe{
            let x_ptr: *mut c_double = transmute(box x);
            let y_ptr: *mut c_double = transmute(box y);

            ffi::cairo_user_to_device(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn user_to_device_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe{
            let dx_ptr: *mut c_double = transmute(box dx);
            let dy_ptr: *mut c_double = transmute(box dy);

            ffi::cairo_user_to_device_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }

    pub fn device_to_user(&self, x: f64, y: f64) -> (f64, f64) {
        unsafe{
            let x_ptr: *mut c_double = transmute(box x);
            let y_ptr: *mut c_double = transmute(box y);

            ffi::cairo_device_to_user(self.get_ptr(), x_ptr, y_ptr);

            let x_box: Box<f64> = transmute(x_ptr);
            let y_box: Box<f64> = transmute(y_ptr);

            (*x_box, *y_box)
        }
    }

    pub fn device_to_user_distance(&self, dx: f64, dy: f64) -> (f64, f64) {
        unsafe{
            let dx_ptr: *mut c_double = transmute(box dx);
            let dy_ptr: *mut c_double = transmute(box dy);

            ffi::cairo_device_to_user_distance(self.get_ptr(), dx_ptr, dy_ptr);

            let dx_box: Box<f64> = transmute(dx_ptr);
            let dy_box: Box<f64> = transmute(dy_ptr);

            (*dx_box, *dy_box)
        }
    }
}
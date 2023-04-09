use std::ffi::{c_float, c_void};

extern "C" {
    pub fn counter_factory() -> *mut c_void;
    pub fn counter_destructor(counter: *mut c_void);
    pub fn counter_inc(counter: *mut c_void);

    pub fn wthread_ctor() -> *mut c_void;
    pub fn wthread_dtor(ptr: *mut c_void);
    pub fn wthread_start(ptr: *mut c_void);
    pub fn wthread_stop(ptr: *mut c_void);

    pub fn get_vec() -> *mut c_float;
    pub fn free_vec(ptr: *mut c_float);
}

pub struct Counter {
    ptr: *mut c_void,
}

impl Counter {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: counter_factory(),
            }
        }
    }
    pub fn inc(&mut self) {
        unsafe {
            counter_inc(self.ptr);
        }
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe { counter_destructor(self.ptr) }
    }
}

pub struct WThread {
    ptr: *mut c_void,
}

impl WThread {
    pub fn new() -> Self {
        unsafe {
            Self {
                ptr: wthread_ctor(),
            }
        }
    }
    pub fn start(&mut self) {
        unsafe { wthread_start(self.ptr) }
    }

    pub fn stop(&mut self) {
        unsafe { wthread_stop(self.ptr) }
    }
}

impl Drop for WThread {
    fn drop(&mut self) {
        unsafe { wthread_dtor(self.ptr) }
    }
}

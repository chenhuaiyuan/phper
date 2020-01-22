use crate::sys::{zend_execute_data, zval};

pub struct Parameters {
    pub(crate) ptr: *mut zend_execute_data,
}

impl Parameters {
    pub fn num_args(&self) -> usize {
        unsafe { (*self.ptr).This.u2.num_args as usize }
    }
}

pub struct Value {
    pub(crate) ptr: *mut zval,
}

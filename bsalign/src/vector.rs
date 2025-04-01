use bsalign_sys::bindings;

macro_rules! generate_vector_type {
    ($struct_name:ident, $c_type:ident, $c_el_type:ident, $init_fn:ident, $free_fn:ident, $clear_fn:ident) => {
        #[derive(Debug)]
        pub(crate) struct $struct_name {
            inner: *mut bindings::$c_type,
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                if self.inner.is_null() {
                    return;
                }
                unsafe {
                    bindings::$free_fn(self.inner);
                }
            }
        }

        #[allow(dead_code)]
        impl $struct_name {
            #[inline(always)]
            pub(crate) fn is_null(&self) -> bool {
                self.inner.is_null()
            }
            #[inline(always)]
            pub(crate) fn as_ptr(&self) -> *mut bindings::$c_type {
                self.inner
            }
            #[inline(always)]
            pub(crate) fn new() -> Self {
                let inner = unsafe { bindings::$init_fn(32) };
                $struct_name { inner }
            }
            #[inline(always)]
            pub(crate) fn with_capacity(capacity: usize) -> Self {
                let inner = unsafe { bindings::$init_fn(capacity) };
                $struct_name { inner }
            }
            #[inline(always)]
            pub(crate) fn empty() -> Self {
                $struct_name {
                    inner: std::ptr::null_mut(),
                }
            }
            #[inline(always)]
            pub(crate) fn from(inner: *mut bindings::$c_type) -> Self {
                $struct_name { inner }
            }
            #[inline(always)]
            pub(crate) fn len(&self) -> usize {
                if self.inner.is_null() {
                    return 0;
                }
                unsafe { (*self.inner).size as usize }
            }
            #[inline(always)]
            pub(crate) fn buffer(&self) -> *mut $c_el_type {
                if self.inner.is_null() {
                    return std::ptr::null_mut();
                }
                unsafe { (*self.inner).buffer }
            }
            #[inline(always)]
            pub(crate) fn clear(&mut self) {
                if self.inner.is_null() {
                    return;
                }
                unsafe {
                    bindings::$free_fn(self.inner);
                    self.inner = std::ptr::null_mut();
                }
            }
        }
    };
}

generate_vector_type!(U1V, u1v, u8, bs_u1v_init, bs_u1v_free, bs_u1v_clear);
generate_vector_type!(U4V, u4v, u32, cigars_init, cigars_free, cigars_clear);

#[derive(Debug)]
pub struct Mempool {
    inner: *mut bindings::b1v,
}

#[allow(dead_code)]
impl Mempool {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        let inner = unsafe { bindings::mempool_init(cap, 0, 0) };
        Mempool { inner }
    }

    pub(crate) fn is_null(&self) -> bool {
        self.inner.is_null()
    }
    #[inline(always)]
    pub(crate) fn as_ptr(&self) -> *mut bindings::b1v {
        self.inner
    }
    #[inline(always)]
    pub(crate) fn clear(&self) {
        if self.inner.is_null() {
            return;
        }
        unsafe {
            bindings::mempool_clear(self.inner);
        }
    }
}

impl Drop for Mempool {
    fn drop(&mut self) {
        if self.inner.is_null() {
            return;
        }
        unsafe {
            bindings::mempool_free(self.inner);
        }
    }
}

use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for HelenOS.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HelenOSDisplayHandle {
    /// A pointer to ui_t
    pub display: NonNull<c_void>,
}

impl HelenOSDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::HelenOSDisplayHandle;
    /// # type ui_t = ();
    /// #
    /// let display: NonNull<ui_t>;
    /// # display = NonNull::from(&());
    /// let handle = HelenOSDisplayHandle::new(display.cast());
    /// ```
    pub fn new(display: NonNull<c_void>) -> Self {
        Self {
            display,
        }
    }
}

/// Raw window handle for HelenOS.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HelenOSWindowHandle {
    /// A pointer to ui_window_t
    pub window: NonNull<c_void>,
}

unsafe impl Send for HelenOSWindowHandle {}
unsafe impl Sync for HelenOSWindowHandle {}

impl HelenOSWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::HelenOSWindowHandle;
    /// # type ui_window_t = ();
    /// #
    /// let window: NonNull<ui_window_t>;
    /// # window = NonNull::from(&());
    /// let handle = HelenOSWindowHandle::new(window.cast());
    /// ```
    pub fn new(window: NonNull<c_void>) -> Self {
        Self {
            window,
        }
    }
}

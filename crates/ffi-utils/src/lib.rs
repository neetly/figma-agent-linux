#[macro_export]
macro_rules! cstr {
    ($($(#[$meta:meta])* $vis:vis const $name:ident = $str:literal;)*) => {
        $(
            $(#[$meta])*
            $vis const $name: &std::ffi::CStr =
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($str, "\0").as_bytes()) };
        )*
    };
}

#[macro_export]
macro_rules! opaque {
    ($($(#[$meta:meta])* $vis:vis struct $name:ident;)*) => {
        $(
            $(#[$meta])*
            $vis struct $name {
                _data: [u8; 0],
                _marker: std::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
            }
        )*
    };
}

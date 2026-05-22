// #[link(name = "user32")] // Link to the User32 library for Windows API functions
// extern "system" {
//     fn MessageBoxA(
//         hWnd: *mut std::ffi::c_void,
//         lpText: *const u8,
//         lpCaption: *const u8,
//         uType: u32,
//     ) -> i32;
// }

// let text = b"Hello\0".as_ptr();
// let caption = b"Hello\0".as_ptr();

// unsafe {
//     MessageBoxA(std::ptr::null_mut(), text, caption, 0);
// }
// 0

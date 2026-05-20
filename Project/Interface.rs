#![no_main]
#![windows_subsystem = "windows"]
#![no_std]

use core::ffi::c_void;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(name = "user32")]
extern "system" {
    fn RegisterClassA(wnd_class: *const WNDCLASS) -> u16;
    fn DefWindowProcA(hwnd: *mut c_void, msg: u32, wparam: usize, lparam: isize) -> isize;
    fn CreateWindowExA(
        dwExStyle: u32,
        lpClassName: *const u8,
        lpWindowName: *const u8,
        dwStyle: u32,
        x: i32,
        y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: *mut c_void,
        hMenu: *mut c_void,
        hInstance: *mut c_void,
        lpParam: *mut c_void,
    ) -> *mut c_void;
    fn GetMessageA(
        lpMsg: *mut MSG,
        hWnd: *mut c_void,
        wMsgFilterMin: u32,
        wMsgFilterMax: u32,
    ) -> i32;
    fn TranslateMessage(lpMsg: *const MSG) -> i32;
    fn DispatchMessageA(lpMsg: *const MSG) -> isize;
    fn PostQuitMessage(nExitCode: i32);
}

const CS_HREDRAW: u32 = 0x0002;
const CS_VREDRAW: u32 = 0x0001;
const CS_OWNDC: u32 = 0x0020;
const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;
const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
const WS_VISIBLE: u32 = 0x10000000;

#[repr(C)]
struct WNDCLASS {
    style: u32,
    lpfn_wnd_proc: extern "system" fn(*mut c_void, u32, usize, isize) -> isize, // -> window function, später umänder in universele
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: *mut c_void,
    h_icon: *mut c_void,
    h_cursor: *mut c_void,
    hbr_background: *mut c_void,
    lpsz_menu_name: *const u8,
    lpsz_class_name: *const u8,
}

impl Default for WNDCLASS {
    fn default() -> Self {
        WNDCLASS {
            style: 0,
            lpfn_wnd_proc: window_callback,
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: core::ptr::null_mut(),
            h_icon: core::ptr::null_mut(),
            h_cursor: core::ptr::null_mut(),
            hbr_background: core::ptr::null_mut(),
            lpsz_menu_name: core::ptr::null(),
            lpsz_class_name: core::ptr::null(),
        }
    }
}

#[repr(C)]
struct MSG {
    hwnd: *mut c_void,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    pt_x: i32,
    pt_y: i32,
    lprivate: u32,
}

impl Default for MSG {
    fn default() -> Self {
        MSG {
            hwnd: core::ptr::null_mut(),
            message: 0,
            wparam: 0,
            lparam: 0,
            time: 0,
            pt_x: 0,
            pt_y: 0,
            lprivate: 0,
        }
    }
}

extern "system" fn window_callback(
    window: *mut c_void,
    message: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    let result: isize = match message {
        0x0005 => {
            // println!("WM_SIZE");
            0
        } // SIZE

        0x0002 => unsafe {
            // println!("WM_DESTROY");
            PostQuitMessage(0);
            0
        }, // DESTROY

        // 0x0010 => {
        //     // println!("WM_CLOSE");
        //     DefWindowProcA(window, message, wparam, lparam);
        //     0
        // } // CLOSE
        0x0006 => {
            // println!("WM_ACTIVATE");
            0
        } // ACTIVATE
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    };
    result
}

#[no_mangle]
pub extern "system" fn WinMain(
    instance: *mut core::ffi::c_void,
    _hprevinstance: *mut core::ffi::c_void,
    _cmdline: *const u8,
    _cmdshow: i32,
) -> i32 {
    let mut exit_code: i32 = 0;
    let mut window_class = WNDCLASS::default();
    window_class.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    window_class.lpfn_wnd_proc = window_callback;
    window_class.h_instance = instance;
    window_class.lpsz_class_name = b"3DRenderer\0".as_ptr();

    unsafe {
        if RegisterClassA(&window_class) != 0 {
            let window_handle = CreateWindowExA(
                0,                                // dwExStyle
                window_class.lpsz_class_name,     // lpClassName
                b"3D Renderer\0".as_ptr(),        // lpWindowName (Titel)
                WS_OVERLAPPEDWINDOW | WS_VISIBLE, // dwStyle = WS_OVERLAPPEDWINDOW | WS_VISIBLE
                CW_USEDEFAULT,                    // x
                CW_USEDEFAULT,                    // y
                CW_USEDEFAULT,                    // nWidth
                CW_USEDEFAULT,                    // nHeight
                core::ptr::null_mut(),            // hWndParent
                core::ptr::null_mut(),            // hMenu
                instance,                         // hInstance
                core::ptr::null_mut(),            // lpParam
            );
            if !window_handle.is_null() {
                let mut message = MSG::default();

                loop {
                    let message_result = GetMessageA(&mut message, core::ptr::null_mut(), 0, 0);
                    if message_result > 0 {
                        TranslateMessage(&message);
                        DispatchMessageA(&message);
                    } else if message_result == 0 {
                        break;
                    } else {
                        exit_code = -1;
                        break;
                    }
                }
            }
        }
    }
    exit_code
}

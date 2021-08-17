extern crate user32;
#[cfg(windows)]
extern crate winapi;

use std::convert::TryInto;
use std::ffi::OsStr;
use std::iter::once;
use std::mem::zeroed;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::windef::{HBITMAP, HBITMAP__, HGDIOBJ, HWND, POINT, RECT, SIZE};
use winapi::um::wingdi::{
    CreateBitmap, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetObjectW,
    SaveDC, SelectObject,
};
use winapi::um::winuser::{
    BringWindowToTop, FindWindowW, GetClientRect, GetDC, GetWindowDC, GetWindowRect, IsRectEmpty,
    ReleaseDC, SendMessageW, SetForegroundWindow, SetWindowPos, ShowWindow, HWND_TOP, HWND_TOPMOST,
    SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SW_NORMAL, VK_SNAPSHOT,
};

// Keyboard Controller
use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};

#[cfg(windows)]
fn active_window(name: &str) -> RECT {
    let window: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();
    let hwnd = unsafe { FindWindowW(null_mut(), window.as_ptr()) };
    let mut my_rect = unsafe { zeroed::<winapi::shared::windef::RECT>() };

    if hwnd != null_mut() {
        println!("Windows Found");

        unsafe {
            SetForegroundWindow(hwnd);
            ShowWindow(hwnd, SW_NORMAL);
            BringWindowToTop(hwnd);
            SetWindowPos(
                hwnd,
                HWND_TOP,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );

            //sleep(Duration::from_millis(500));

            //let _client_rect = GetClientRect(hwnd, &mut my_rect);
            let _window_rect = GetWindowRect(hwnd, &mut my_rect);
        }
    } else {
        println!("Not Found");
    }
    return my_rect;
}

#[cfg(windows)]
fn send_print(name: &str) {
    use std::convert::TryInto;

    let window: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();
    let hwnd = unsafe { FindWindowW(null_mut(), window.as_ptr()) };
    let mut my_rect = unsafe { zeroed::<winapi::shared::windef::RECT>() };

    if hwnd != null_mut() {
        unsafe {
            SendMessageW(hwnd, VK_SNAPSHOT.try_into().unwrap(), 0, 0);
        }
    }
}

fn main() {
    NumLockKey.bind(|| {
        let mut isFirst = false;
        let mut my_rect = unsafe { zeroed::<winapi::shared::windef::RECT>() };
        while NumLockKey.is_toggled() {
            if !isFirst {
                println!("isFirst");
                my_rect = active_window("12TailsTH"); // just for test
                isFirst = true;
                //sleep(Duration::from_millis(1500));
            }
            let hasRect = unsafe { IsRectEmpty(&my_rect) } == 0;
            println!("is REct == {}", hasRect);
            if hasRect {
                // println!(
                //     "Rect is T{} L{} R{} {}",
                //     my_rect.top, my_rect.left, my_rect.right, my_rect.bottom
                // );
                // let w = my_rect.right - my_rect.left;
                // let h = my_rect.bottom - my_rect.top;
                // //println!("RECT W{} H{}", h, w);
                //RightButton.press();
                //MouseCursor::move_abs(my_rect.left + w / 2, my_rect.top + h / 2);
                OtherKey(VK_SNAPSHOT.try_into().unwrap()).press();
                //send_print("12TailsTH");
                sleep(Duration::from_millis(10));
                //MouseCursor::move_abs(my_rect.left + w/2+200, my_rect.top + h/2);
                //MouseCursor::move_rel(200, 200);
                //sleep(Duration::from_millis(150));
                //RightButton.release();
            }
        }
        //RightButton.release();
    });
    handle_input_events();
}

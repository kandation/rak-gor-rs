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
use std::{thread::sleep, time::Duration, time::Instant};

#[cfg(windows)]
fn active_window(name: &str) -> RECT {
    let window: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();
    let hwnd = unsafe { FindWindowW(null_mut(), window.as_ptr()) };
    let mut my_rect = unsafe { zeroed::<winapi::shared::windef::RECT>() };

    if hwnd != null_mut() {
        println!("Game Found");

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
        println!("Game Not Found");
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
    CapsLockKey.bind(|| {
        let mut isFirst = false;
        let mut my_rect = unsafe { zeroed::<winapi::shared::windef::RECT>() };
        let ten_millis = Duration::from_millis(1000);
        let mut prev_time = Instant::now();

        while CapsLockKey.is_toggled() {
            if !isFirst {
                my_rect = active_window("12TailsTH"); // just for test
                isFirst = true;
                println!("RakGor is Activate process every 60 sec");
                //sleep(Duration::from_millis(1500));
            }
            let hasRect = unsafe { IsRectEmpty(&my_rect) } == 0;
            //println!(" {}", hasRect);
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
                
                if prev_time.elapsed() >= Duration::from_secs(60) {
                    prev_time = Instant::now();
                    OtherKey(VK_SNAPSHOT.try_into().unwrap()).release();

                    println!("GG");
                    sleep(Duration::from_millis(100));
                }else{
                    OtherKey(VK_SNAPSHOT.try_into().unwrap()).press();
                }
               
                //send_print("12TailsTH");
                sleep(Duration::from_millis(10));
                //MouseCursor::move_abs(my_rect.left + w/2+200, my_rect.top + h/2);
                //MouseCursor::move_rel(200, 200);
                //sleep(Duration::from_millis(150));
                //RightButton.release();
            }
        }
        //RightButton.release();
        println!("RakGor is De-Activate process");
    });
    handle_input_events();
}

#![allow(unused)]
#![allow(non_camel_case_types)]

use super::types::*;

#[link(name = "user32", kind = "dylib")]
extern "system" {

    pub fn CreateDesktopA(
        lpszDesktop: LPCSTR,
        lpszDevice: LPCSTR,
        pDevmode: PDEVMODEA,
        dwFlags: DWORD,
        dwDesiredAccess: ACCESS_MASK,
        lpsa: LPVOID,
    ) -> HDESK;

    pub fn OpenDesktopA(
        lpszDesktop: LPCSTR,
        dwFlags: DWORD,
        fInherit: BOOL,
        dwDesiredAccess: ACCESS_MASK,
    ) -> HDESK;

    pub fn CloseDesktop(hDesktop: HDESK) -> BOOL;

    pub fn SetThreadDesktop(hDesktop: HDESK) -> BOOL;

    pub fn EnumDesktopWindows(
        hDesktop: HDESK,
        lpfn: extern "C" fn(LPVOID, LPVOID),
        lParm: LPVOID,
    ) -> BOOL;

    pub fn GetDesktopWindow() -> HWND;

    pub fn GetWindowRect(hwnd: HWND, rect: *mut Rect) -> BOOL;

    pub fn GetDC(hwnd: HWND) -> HDC;
}

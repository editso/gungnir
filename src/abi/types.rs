#![allow(unused)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub type LPVOID = *mut c_void;
pub type HDESK = LPVOID;
pub type LPCSTR = *const u8;
pub type DWORD = u32;
pub type BOOL = bool;
pub type ACCESS_MASK = DWORD;
pub type LPSTR = LPCSTR;
pub type WORD = u16;
pub type LPBYTE = *mut u8;
pub type HANDLE = LPVOID;
pub type HWND = HANDLE;
pub type LONG = std::os::raw::c_long;
pub type HDC = HANDLE;
pub type HBITMAP = LPVOID;
pub type HGDIOBJ = HBITMAP;

pub type PDEVMODEA = LPVOID;

pub const GENERIC_READ: DWORD = 0x80000000;
pub const GENERIC_WRITE: DWORD = 0x40000000;
pub const GENERIC_EXECUTE: DWORD = 0x20000000;
pub const GENERIC_ALL: DWORD = 0x10000000;

pub type PSTARTUPINFOA = *mut STARTUPINFOA;

c2rs::c2rs_def!(

    struct STARTUPINFOA {
        DWORD   cb;
        LPSTR   lpReserved;
        LPSTR   lpDesktop;
        LPSTR   lpTitle;
        DWORD   dwX;
        DWORD   dwY;
        DWORD   dwXSize;
        DWORD   dwYSize;
        DWORD   dwXCountChars;
        DWORD   dwYCountChars;
        DWORD   dwFillAttribute;
        DWORD   dwFlags;
        WORD    wShowWindow;
        WORD    cbReserved2;
        LPBYTE  lpReserved2;
        HANDLE  hStdInput;
        HANDLE  hStdOutput;
        HANDLE  hStdError;
    };

    struct PROCESS_INFORMATION {
        HANDLE hProcess;
        HANDLE hThread;
        DWORD dwProcessId;
        DWORD dwThreadId;
    };

    struct Rect{
        LONG    left;
        LONG    top;
        LONG    right;
        LONG    bottom;
    };
);

use super::types::{BOOL, DWORD, LPCSTR, LPVOID, PROCESS_INFORMATION, PSTARTUPINFOA};

extern "system" {

    pub fn CreateProcessA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPCSTR,
        lpProcessAttributes: LPVOID,
        lpThreadAttributes: LPVOID,
        bInherHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: PSTARTUPINFOA,
        lpProcessInformation: *mut PROCESS_INFORMATION,
    ) -> BOOL;

}

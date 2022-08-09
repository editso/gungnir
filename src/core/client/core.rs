use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use futures::{AsyncRead, AsyncWrite};

use crate::abi::desktop::{GetDC, GetDesktopWindow, GetWindowRect};
use crate::abi::gdi::{CreateCompatibleBitmap, CreateCompatibleDC, SelectObject};
use crate::abi::types::HWND;
use crate::core::packet::Message;
use crate::core::rect::Rect;
use crate::error::Result;
use crate::{abi, error};

#[derive(Debug)]
pub struct HDESK(crate::abi::types::HDESK);

#[derive(Debug)]
#[allow(unused)]
pub struct Desktop {
    raw: HDESK,
    name: String,
    rect: Rect,
}

#[allow(unused)]
pub struct Client<S> {
    stream: S,
    desktop: Arc<Desktop>,
}

unsafe impl Send for HDESK {}

unsafe impl Sync for HDESK {}

impl Desktop {
    #[allow(non_snake_case)]
    pub fn open<DesktopName, Size>(name: DesktopName, rect: Size) -> Result<Self>
    where
        DesktopName: Into<String>,
        Size: Into<Rect>,
    {
        let desktop_name = format!("{}\0", name.into());

        let raw = unsafe {
            let mut hDesk =
                abi::desktop::OpenDesktopA(desktop_name.as_ptr(), 0, true, abi::types::GENERIC_ALL);

            if hDesk.is_null() {
                hDesk = abi::desktop::CreateDesktopA(
                    desktop_name.as_ptr(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    0,
                    abi::types::GENERIC_ALL,
                    std::ptr::null_mut(),
                );
            }

            if hDesk.is_null() {
                return Err(error::Error::open_desktop_error());
            }

            HDESK(hDesk)
        };

        Ok(Self {
            raw: raw,
            name: desktop_name,
            rect: rect.into(),
        })
    }
}

impl Drop for HDESK {
    fn drop(&mut self) {
        unsafe {
            abi::desktop::CloseDesktop(std::mem::replace(&mut self.0, std::ptr::null_mut()));
        }
    }
}

impl Deref for HDESK {
    type Target = abi::types::HDESK;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HDESK {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Desktop {
    pub fn set_thread(&self) -> Result<()> {
        unsafe {
            if !abi::desktop::SetThreadDesktop(self.raw.0) {
                Err(error::Error::set_thread_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn start_explorer(&self) -> Result<()> {
        Ok(())
    }

    pub fn capture<R>(&self, rect: R) -> Result<Vec<u8>>
    where
        R: Into<Rect>,
    {
        unsafe {
            let hWndDesktop = GetDesktopWindow();
            let mut rect: abi::types::Rect = std::mem::zeroed();

            if !GetWindowRect(hWndDesktop, &mut rect) {
                return Err(error::Error::os_error());
            };

            let hdc = GetDC(std::ptr::null_mut());
            let hdcScreen = CreateCompatibleDC(hdc);
            let hBmpScreen = CreateCompatibleBitmap(hdc, rect.right, rect.left);

            SelectObject(hdcScreen, hBmpScreen);

            

            println!("{:?}", rect);
        }

        todo!()
    }

    pub fn spawn(&self, prog: &str, args: &str) -> Result<()> {
        unsafe {
            let mut si: abi::types::STARTUPINFOA = std::mem::zeroed();
            let mut pi: abi::types::PROCESS_INFORMATION = std::mem::zeroed();

            si.cb = abi::types::STARTUPINFOA::size() as u32;

            si.lpDesktop = self.name.as_ptr();

            let proc = abi::process::CreateProcessA(
                std::ptr::null(),
                prog.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                false,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut si,
                &mut pi,
            );

            if !proc {
                return Err(error::Error::spawn_process_error());
            }
            Ok(())
        }
    }
}

impl<S> Deref for Client<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<S> DerefMut for Client<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}

impl<S> Client<S>
where
    S: AsyncRead + AsyncWrite + Send + Sync + 'static,
{
    pub async fn connect(desktop: Desktop, stream: S) -> Result<Self> {
        desktop.set_thread()?;

        Ok(Self {
            stream,
            desktop: Arc::new(desktop),
        })
    }

    pub async fn next(&mut self) -> Result<Message> {
        todo!()
    }
}

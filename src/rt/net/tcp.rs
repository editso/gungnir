use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use futures::{AsyncRead, AsyncWrite};

pub struct TcpStream<Inner>(Inner);

pub trait AsTcpStream<T> {
    fn as_tcp_stream(self) -> TcpStream<T>;
}

impl<T> AsyncWrite for TcpStream<T>
where
    T: tokio::io::AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        tokio::io::AsyncWrite::poll_write(Pin::new(&mut self.0), cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_flush(Pin::new(&mut self.0), cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        tokio::io::AsyncWrite::poll_shutdown(Pin::new(&mut self.0), cx)
    }
}

impl<T> AsyncRead for TcpStream<T>
where
    T: tokio::io::AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        
        unimplemented!()
    }
}

impl<T> AsTcpStream<T> for T
where
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + 'static,
{
    fn as_tcp_stream(self) -> TcpStream<T> {
        TcpStream(self)
    }
}

impl<T> Deref for TcpStream<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TcpStream<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

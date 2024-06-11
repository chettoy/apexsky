mod kmbox_b;
mod kmbox_net;

use std::io;

pub use kmbox_b::KmboxB;
pub use kmbox_net::{KmboxNet, SoftMouse};

#[derive(thiserror::Error, Debug)]
pub enum KmboxError {
    #[error("KmboxErr: serialport error")]
    SerialPort(#[from] tokio_serial::Error),
    #[error("KmboxErr: serialport io error")]
    SerialPortIO(io::Error),
    #[error("KmboxErr: failed to create socket")]
    CreateSocket(io::Error), //= -9000,	//创建socket失败
    #[error("KmboxErr: wrong socket version")]
    NetVersion, //socket版本错误
    #[error("KmboxErr: socket send error")]
    NetTx(io::Error), //socket发送错误
    #[error("KmboxErr: socket recv error")]
    NetRx(io::Error), //socket接收错误
    #[error("KmboxErr: socket recv timeout")]
    NetRxTimeout(tokio::time::error::Elapsed), //socket接收超时
    #[error("KmboxErr: wrong command")]
    NetCmd, //命令错误
    #[error("KmboxErr: wrong timestamp")]
    NetPts, //时间戳错误
    #[error("KmboxErr: USB device send timeout")]
    UsbDevTxTimeout, //USB device发送失败
    #[error("KmboxErr: error {0:?}")]
    AnyError(#[from] anyhow::Error),
}

use std::fmt::Debug;

use obfstr::obfstr as s;
use once_cell::sync::Lazy;
use serial2_tokio::SerialPort;

use super::KmboxError;

pub struct KmboxB {
    serial: SerialPort,
}

impl Debug for KmboxB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KmboxB")
            .field("serial(..)", &self.serial.get_configuration())
            .finish()
    }
}

impl KmboxB {
    pub fn print_serial_ports() {
        let ports = SerialPort::available_ports().unwrap_or_default();
        for p in ports {
            println!("{}", p.to_string_lossy());
        }
    }

    pub async fn init(serial_port: &str, baud_rate: u32) -> Result<Self, KmboxError> {
        let port = SerialPort::open(serial_port, baud_rate).map_err(KmboxError::SerialPortIO)?;

        let mut instance = Self { serial: port };
        instance.execute_command(s!("km.version()")).await?;
        instance.execute_command(s!("km.MAC()")).await?;

        Ok(instance)
    }

    async fn execute_command(&mut self, cmd: &str) -> Result<(), KmboxError> {
        self.serial
            .write(format!("{}\r\n", cmd).as_bytes())
            .await
            .map_err(KmboxError::SerialPortIO)?;

        let mut buffer = [0; 1024];
        self.serial
            .read(&mut buffer)
            .await
            .map_err(KmboxError::SerialPortIO)?;
        let output = String::from_utf8_lossy(&buffer);
        tracing::trace!(?cmd, ?output);
        Ok(())
    }

    pub async fn mouse_move(&mut self, x: i16, y: i16) -> Result<(), KmboxError> {
        static FN_MOVE: Lazy<String> = Lazy::new(|| s!("km.move(").to_owned());
        let cmd = format!("{}{}{}{}{}", *FN_MOVE, x, ",", y, ")");
        self.execute_command(&cmd).await?;
        Ok(())
    }

    pub async fn mouse_left(&mut self, is_down: bool) -> Result<(), KmboxError> {
        static FN_LEFT: Lazy<String> = Lazy::new(|| s!("km.left(").to_owned());
        let cmd = format!("{}{}{}", *FN_LEFT, if is_down { 1 } else { 0 }, ")");
        self.execute_command(&cmd).await?;
        Ok(())
    }
}

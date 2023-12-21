// MIT License
//
// Copyright (c) 2023 chettoy
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::time::Duration;

use zbus::{dbus_proxy, Result};

#[dbus_proxy(
    interface = "org.freedesktop.UPower.KbdBacklight",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/KbdBacklight"
)]
trait KbdBacklight {
    fn GetMaxBrightness(&self) -> Result<i32>;
    fn GetBrightness(&self) -> Result<i32>;
    fn SetBrightness(&self, brightness: i32) -> Result<()>;
}

pub struct SysContext {
    zbus_conn: zbus::blocking::Connection,
}

impl SysContext {
    pub fn new() -> anyhow::Result<Self> {
        let zbus_conn = zbus::blocking::Connection::system()?;

        Ok(SysContext { zbus_conn })
    }

    fn kbd_backlight_proxy(&self) -> Result<KbdBacklightProxyBlocking> {
        KbdBacklightProxyBlocking::new(&self.zbus_conn)
    }

    /// Blink the keyboard backlight a specified number of times.
    ///
    /// This function uses D-Bus to control the keyboard backlight via the
    /// `org.freedesktop.UPower.KbdBacklight` interface and synchronously blinks it for a
    /// specified number of times.
    ///
    /// The function will block until the blinking operation is completed.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of times to blink the keyboard backlight.
    ///
    /// # Examples
    ///
    /// ```
    /// // Blink the keyboard backlight three times
    /// kbd_blink(3);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue communicating with D-Bus, if the
    /// UPower service is not available, or if controlling the keyboard backlight fails.
    ///
    /// # Safety
    ///
    /// This function may manipulate hardware indirectly through D-Bus and should be used with caution.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the blinking operation is successful.
    /// Returns an `Err` variant with an error description if an issue occurs.

    pub fn kbd_blink(&self, count: u8) -> Result<()> {
        use std::thread::sleep;
        let kbd_backlight = self.kbd_backlight_proxy()?;
        let curr_brightness = kbd_backlight.GetBrightness()?;
        let max_brightness = kbd_backlight.GetMaxBrightness()?;

        let (toggle_1, toggle_0) = if curr_brightness == 0 {
            (max_brightness, 0)
        } else {
            (0, curr_brightness)
        };

        fn blink(
            n: u8,
            kbd_backlight: &KbdBacklightProxyBlocking,
            toggle_0: i32,
            toggle_1: i32,
        ) -> Result<()> {
            let toggle_for = |millis: u64| -> Result<()> {
                kbd_backlight.SetBrightness(toggle_1)?;
                sleep(Duration::from_millis(millis));
                kbd_backlight.SetBrightness(toggle_0)?;
                Ok(())
            };
            match n {
                0 => (),
                1 => {
                    toggle_for(400)?;
                    sleep(Duration::from_millis(400));
                }
                2 => {
                    for _ in 0..2 {
                        toggle_for(200)?;
                        sleep(Duration::from_millis(150));
                    }
                }
                3 => {
                    for _ in 0..3 {
                        toggle_for(150)?;
                        sleep(Duration::from_millis(100));
                    }
                }
                4 => {
                    for _ in 0..2 {
                        for _ in 0..2 {
                            toggle_for(150)?;
                            sleep(Duration::from_millis(100));
                        }
                        sleep(Duration::from_millis(50));
                    }
                }
                5 => {
                    blink(3, kbd_backlight, toggle_0, toggle_1)?;
                    sleep(Duration::from_millis(100));
                    for _ in 0..2 {
                        toggle_for(150)?;
                        sleep(Duration::from_millis(100));
                    }
                    sleep(Duration::from_millis(100));
                }
                6 => {
                    for _ in 0..2 {
                        blink(3, kbd_backlight, toggle_0, toggle_1)?;
                        sleep(Duration::from_millis(150));
                    }
                }
                7 => {
                    blink(4, kbd_backlight, toggle_0, toggle_1)?;
                    sleep(Duration::from_millis(150));
                    blink(3, kbd_backlight, toggle_0, toggle_1)?;
                    sleep(Duration::from_millis(150));
                }
                _ => {
                    for _ in 0..n {
                        toggle_for(200)?;
                        sleep(Duration::from_millis(150));
                    }
                }
            }
            Ok(())
        }

        blink(count, &kbd_backlight, toggle_0, toggle_1)
    }

    pub fn kbd_backlight_test(&self) -> Result<()> {
        self.kbd_blink(2)?;

        Ok(())
    }
}

#[test]
fn test_kdb_backlight() {
    // tokio::runtime::Runtime::new()
    //     .expect("Failed to create tokio runtime")
    //     .block_on(kbd_backlight_test())
    //     .expect("Error listening to signal");
    // SysContext::new().unwrap().kbd_backlight_test().unwrap();
    let ctx = SysContext::new().unwrap();
    for i in 1..10 {
        println!("blink {}", i);
        ctx.kbd_blink(i).unwrap();
        std::thread::sleep(Duration::from_millis(1000));
    }
}

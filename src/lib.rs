use embedded_graphics_core::{draw_target::DrawTarget, geometry::{Dimensions, Point, Size}, primitives::Rectangle, Pixel};

pub const DEVICE_ID_APEX_PRO: u16 = 0x1610;

const VENDOR_ID: u16 = 0x1038;
const OLED_WIDTH: u32 = 128;
const OLED_HEIGHT: u32 = 40;
const BUFFER_SIZE: u32 = OLED_WIDTH * OLED_HEIGHT / 8;

pub struct SteelSeriesApexOled {
    device: rusb::DeviceHandle<rusb::GlobalContext>,
    buffer: [u8; BUFFER_SIZE as usize],
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DeviceNotFound,
    RusbError(rusb::Error),
}

impl SteelSeriesApexOled {
    pub fn new(device_id: u16) -> Result<Self, Error> {
        let mut keyboard_device = None;

        let devices = match rusb::devices() {
            Ok(devices) => devices,
            Err(err) => return Err(Error::RusbError(err))
        };

        for device in devices.iter() {
            let device_desc = match device.device_descriptor() {
                Ok(device_desc) => device_desc,
                Err(_) => continue
            };

            if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == device_id {
                keyboard_device = Some(device);
            }
        }

        let keyboard_device = match keyboard_device {
            Some(keyboard_device) => keyboard_device,
            None => return Err(Error::DeviceNotFound)
        };
        let mut keyboard_device_handle = match keyboard_device.open() {
            Ok(handle) => handle,
            Err(err) => return Err(Error::RusbError(err))
        };
        match keyboard_device_handle.claim_interface(1) {
            Ok(_) => { },
            Err(err) => return Err(Error::RusbError(err))
        }

        Ok(Self {
            device: keyboard_device_handle,
            buffer: [0; BUFFER_SIZE as usize],
        })
    }

    pub fn flush(&self) -> Result<(), Error> {
        let mut out_buffer = [0; BUFFER_SIZE as usize + 2];
        out_buffer[0] = 0x65;
        let buffer_len = out_buffer.len();
        out_buffer[1..buffer_len - 1].copy_from_slice(&self.buffer);

        let result = self.device.write_control(0x21, 0x09, 0x0300, 0x0001, &out_buffer, std::time::Duration::from_millis(1000));

        match result {
            Ok(_) => Ok(()),
            Err(err) => return Err(Error::RusbError(err))
        }
    }
}

impl Dimensions for SteelSeriesApexOled {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(OLED_WIDTH, OLED_HEIGHT))
    }
}

impl DrawTarget for SteelSeriesApexOled {
    type Color = embedded_graphics_core::pixelcolor::BinaryColor;

    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels.into_iter() {
            if let Ok((x @ 0..=OLED_WIDTH, y @ 0..=OLED_HEIGHT)) = coord.try_into() {
                let byte_index = (OLED_WIDTH & y + x) / 8;
                let bit_index = 7 - (x % 8);
                let byte = &mut self.buffer[byte_index as usize];
                if color == embedded_graphics_core::pixelcolor::BinaryColor::On {
                    *byte |= 1 << bit_index;
                } else {
                    *byte &= !(1 << bit_index);
                }
            }
        }

        Ok(())
    }
}
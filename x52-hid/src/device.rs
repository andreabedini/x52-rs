//! X52 input device handling

use anyhow::Result;
use log::{debug, trace};

use crate::error::Error;
use crate::input::{Axis, AxisRange};
use crate::parser::{parse_x52, parse_x52_pro};
use crate::report::{AxisRanges, InputReport};
use async_hid::{AsyncHidRead, HidBackend};
use async_io::Timer;
use futures_lite::FutureExt;
use futures_lite::stream::StreamExt;
use smol::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;
use x52_device::DeviceVariant;
use x52_device::device::{PRODUCT_IDS, VENDOR_SAITEK};

const USAGE_PAGE: u16 = 0x0001;
const USAGE_ID: u16 = 0x0004;

/// X52/X52 Pro input device
pub struct X52Device {
    // device: async_hid::Device,
    handle: async_hid::DeviceReaderWriter,
    variant: DeviceVariant,
    axis_ranges: AxisRanges,
    current_report: Arc<Mutex<InputReport>>,
}

impl X52Device {
    /// Open first available X52/X52 Pro device
    pub async fn open() -> Result<Self> {
        debug!("Scanning for X52/X52 Pro devices");

        let backend = HidBackend::default();

        let (device, variant) = backend
            .enumerate()
            .await?
            .filter_map(|device| {
                debug!(
                    "Found device: VID={:04x}, PID={:04x}, Usage ID={:04x}, Usage Page={:04x}",
                    device.vendor_id, device.product_id, device.usage_id, device.usage_page
                );
                let pid = PRODUCT_IDS
                    .iter()
                    .find(|&&pid| device.matches(USAGE_PAGE, USAGE_ID, VENDOR_SAITEK, pid))?;

                let variant = DeviceVariant::from_product_id(*pid)?;

                Some((device, variant))
            })
            .next()
            .await
            .ok_or(Error::DeviceNotFound)?;

        let handle = device.open().await?;

        let axis_ranges = AxisRanges::for_variant(variant);

        let current_report = Arc::new(Mutex::new(InputReport::new()));

        Ok(Self {
            // device,
            variant,
            axis_ranges,
            current_report,
            handle,
        })
    }

    /// Read next input report (blocking until data available)
    pub async fn read(&mut self) -> Result<InputReport> {
        self.read_timeout(None).await
    }

    /// Read next input report with timeout
    pub async fn read_timeout(&mut self, timeout: Option<Duration>) -> Result<InputReport> {
        let expected_len = match self.variant {
            DeviceVariant::X52 => 14,
            DeviceVariant::X52Pro => 15,
        };

        // Create a buffer for the HID report
        // Add extra byte for potential report ID
        let mut buffer = vec![0u8; expected_len + 1];

        let f = async {
            self.handle
                .read_input_report(&mut buffer)
                .await
                .map_err(|e| e.into())
        };

        let bytes_read = match timeout {
            Some(dur) => {
                f.or(async {
                    Timer::after(dur).await;
                    Err(Error::Timeout)
                })
                .await
            }
            None => f.await,
        }?;

        // Trim buffer to actual bytes read
        buffer.truncate(bytes_read);

        trace!(
            "Read {} bytes from HID device (expected {})",
            bytes_read, expected_len
        );

        // Parse the report
        let mut report = self.current_report.lock().await;

        match self.variant {
            DeviceVariant::X52 => parse_x52(&buffer, &mut report)?,
            DeviceVariant::X52Pro => parse_x52_pro(&buffer, &mut report)?,
        }

        trace!("Parsed report: {:?}", report);

        Ok(report.clone())
    }

    /// Get device variant
    pub fn variant(&self) -> DeviceVariant {
        self.variant
    }

    /// Get axis range for specific axis
    pub fn axis_range(&self, axis: Axis) -> AxisRange {
        self.axis_ranges.get(axis)
    }
}

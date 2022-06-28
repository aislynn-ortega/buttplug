// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2022 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use crate::{
  core::{errors::ButtplugDeviceError, messages::Endpoint},
  server::device::{
    hardware::{HardwareCommand, HardwareWriteCmd},
    protocol::{generic_protocol_setup, ProtocolHandler},
  },
};

generic_protocol_setup!(Realov, "realov");

#[derive(Default)]
pub struct Realov {}

impl ProtocolHandler for Realov {
  fn handle_scalar_vibrate_cmd(
    &self,
    _index: u32,
    scalar: u32
  ) -> Result<Vec<HardwareCommand>, ButtplugDeviceError> {
    Ok(vec![HardwareWriteCmd::new(Endpoint::Tx, [0xc5u8, 0x55, scalar as u8, 0xaa].to_vec(), false).into()])
  }
}

// TODO Write Tests

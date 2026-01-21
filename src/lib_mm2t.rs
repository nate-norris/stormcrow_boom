//! # lib_mm2t
//!
//! `lib_mm2t` provides an abstraction layer for interacting with the 900 MHz
//! serial radio in a Tokio async contexts.
//!
//! This library exposes:
//! - [`PacketT`]: the trait defining a packet to be sent as bytes over the MM2T port
//! - [`MM2TTransport`]: a real hardware implementation using a serial port.

pub mod mm2t;
pub mod mm2t_packet;

// re-export commmon types and functions
pub use mm2t::MM2TTransport;
pub use mm2t_packet::PacketT;
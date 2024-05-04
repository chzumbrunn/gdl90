use std::fmt::Debug;

use crate::error::Gdl90Error;

pub trait Message: Debug {

}

#[derive(Debug)]
pub enum Gdl90Message {
    Heartbeat(Heartbeat),
    OwnShip(OwnShipTraffic),
    Traffic(OwnShipTraffic),
}

impl Gdl90Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Gdl90Message, Gdl90Error> {
        // TODO: error handling, return Result
        match bytes[0] {
            0x00 => {
                Ok(Gdl90Message::Heartbeat(Heartbeat::from_bytes(&bytes[1..])))
            },
            0x10 => {
                Ok(Gdl90Message::OwnShip(OwnShipTraffic::from_bytes(&bytes[1..])?))
            },
            0x20 => {
                Ok(Gdl90Message::Traffic(OwnShipTraffic::from_bytes(&bytes[1..])?))
            },
            _ => {
                Err(Gdl90Error::UnknownMessageType)
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Heartbeat {
    status1: u8,
    status2: u8,
    timestamp: u16,
    uat_message_count: u16,
}

impl Heartbeat {
    pub fn new(status1: u8, status2: u8, timestamp: u16, uat_message_count: u16) -> Heartbeat {
        Heartbeat {
            status1,
            status2,
            timestamp,
            uat_message_count,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Heartbeat {
        // TODO: error handling
        if bytes.len() != 6 { panic!("Invalid size for Heartbeat message!"); }
        Heartbeat {
            status1: bytes[0],
            status2: bytes[1],
            timestamp: (bytes[3] as u16) << 8 | bytes[2] as u16,
            uat_message_count: (bytes[5] as u16) << 8 | bytes[4] as u16,
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct OwnShipTraffic {
    traffic_alert: bool,
    address_type: AddressType,
    address: u32,
    latitude: f32,
    longitude: f32,
    altitude: u32,
    airborne: bool,
    extrapolated: bool,
    track_heading: TrackHeading,
    nic_nacp: u8,
    horizontal_velocity: Option<u16>,
    vertical_speed: Option<i16>,
    emitter_category: u8,
    callsign: [u8; 8],
    emergency: u8,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum AddressType {
    AdsBIcao,
    AdsBSelfAssigned,
    TisBIcao,
    TisBTrackFile,
    SurfaceVehicle,
    GroundStationBeacon,
}

impl AddressType {
    pub fn from_byte(byte: u8) -> Result<AddressType, Gdl90Error> {
        match byte {
            0x0 => Ok(AddressType::AdsBIcao),
            0x1 => Ok(AddressType::AdsBSelfAssigned),
            0x2 => Ok(AddressType::TisBIcao),
            0x3 => Ok(AddressType::TisBTrackFile),
            0x4 => Ok(AddressType::SurfaceVehicle),
            0x5 => Ok(AddressType::GroundStationBeacon),
            0x6..=0xF => Err(Gdl90Error::ReservedContent),
            0x10..=0xFF => Err(Gdl90Error::LogicError)
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum TrackHeading {
    Invalid,
    TrueTrack(f32),
    MagneticHeading(f32),
    TrueHeading(f32),
}

impl OwnShipTraffic {
    pub fn from_bytes(bytes: &[u8]) -> Result<OwnShipTraffic, Gdl90Error> {
        if bytes.len() != 27 { return Err(Gdl90Error::InvalidMessage); }
        Ok(OwnShipTraffic {
            traffic_alert: bytes[0] >> 4 == 1,
            address_type: AddressType::from_byte(bytes[0] & 0xF)?,
            address: Self::u24_from_bytes_msb(&bytes[1..=3]),
            latitude: Self::lat_long_from_bytes(&bytes[4..=6]),
            longitude: Self::lat_long_from_bytes(&bytes[7..=9]),
            altitude: Self::upper_u12_from_bytes_msb(&bytes[10..=11]) as u32 * 25 - 1000,
            airborne: bytes[11] & 0x8 != 0,
            extrapolated: bytes[11] & 0x4 != 0,
            nic_nacp: bytes[12],
            horizontal_velocity: {
                let value = Self::upper_u12_from_bytes_msb(&bytes[13..=14]);
                if value == 0xFFF { None } else { Some(value) }
            },
            vertical_speed: {
                let value = Self::lower_i12_from_bytes_msb(&bytes[14..=15]);
                if value == -0xFFF { None } else { Some(value * 64)}
            },
            track_heading: {
                let value = bytes[16] as f32 * 360.0 / 256.0;
                match bytes[11] & 0x3 {
                    0x0 => Ok(TrackHeading::Invalid),
                    0x1 => Ok(TrackHeading::TrueTrack(value)),
                    0x2 => Ok(TrackHeading::MagneticHeading(value)),
                    0x3 => Ok(TrackHeading::TrueHeading(value)),
                    0x4..=0xFF => Err(Gdl90Error::LogicError)
                }
            }?,
            emitter_category: bytes[17],
            callsign: bytes[18..=25].try_into()?,
            emergency: bytes[26]
        })
    }

    fn u24_from_bytes_msb(bytes: &[u8]) -> u32 {
        (bytes[0] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[2] as u32)
    }

    fn i24_from_bytes_msb(bytes: &[u8]) -> i32 {
        ((Self::u24_from_bytes_msb(bytes) << 8) as i32) >> 8
    }

    fn lat_long_from_bytes(bytes: &[u8]) -> f32 {
        Self::i24_from_bytes_msb(bytes) as f32 * 180.0 / 8388608.0
    }

    fn upper_u12_from_bytes_msb(bytes: &[u8]) -> u16 {
        /* vv vx -> 0vvv */
        (bytes[0] as u16) << 4 | (bytes[1] as u16) >> 4
    }

    fn lower_u12_from_bytes_msb(bytes: &[u8]) -> u16 {
        /* xv vv -> 0vvv */
        ((bytes[0] & 0x0F) as u16) << 8 | (bytes[1] as u16)
    }

    fn lower_i12_from_bytes_msb(bytes: &[u8]) -> i16 {
        ((Self::lower_u12_from_bytes_msb(bytes) << 4) as i16) >> 4
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Gdl90Error;
    use super::*;

    #[test]
    fn test_traffic_report() -> Result<(), Gdl90Error> {
        let report = OwnShipTraffic::from_bytes(&[
            0x00, 0xAB, 0x45, 0x49, 0x1F, 0xEF, 0x15, 0xA8, 0x89, 0x78, 0x0F, 0x09, 0xA9, 0x07, 0xB0, 0x01, 0x20, 0x01, 0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20, 0x00
        ])?;
        
        assert!(!report.traffic_alert);
        assert_eq!(report.address_type, AddressType::AdsBIcao);
        assert_eq!(report.address, 0o52642511);
        assert_eq!(report.latitude, 44.907066);
        assert_eq!(report.longitude, -122.99486);
        assert_eq!(report.altitude, 5000);
        assert!(report.airborne);
        assert!(!report.extrapolated);
        // TODO: NIC NACp
        assert_eq!(report.horizontal_velocity, Some(123));
        assert_eq!(report.vertical_speed, Some(64));
        assert_eq!(report.track_heading, TrackHeading::TrueTrack(45.0));
        assert_eq!(report.emitter_category, 1);
        assert_eq!(report.callsign, [0x4E, 0x38, 0x32, 0x35, 0x56, 0x20, 0x20, 0x20]);
        assert_eq!(report.emergency, 0);

        Ok(())
    }
}
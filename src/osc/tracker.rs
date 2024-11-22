use glam::{
    f32::{Quat, Vec3},
    EulerRot,
};
use rosc::{OscArray, OscMessage, OscPacket, OscType};
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub struct Tracker {
    pub name: String,
    pub position: Vec3,
    pub rotation: Quat,
}

impl Tracker {
    pub fn new(name: &str) -> Tracker {
        Tracker {
            name: String::from(name),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn get_position_addr(self: &Tracker) -> String {
        format!("/tracking/trackers/{}/position", self.name)
    }

    pub fn get_rotation_addr(self: &Tracker) -> String {
        format!("/tracking/trackers/{}/rotation", self.name)
    }

    pub fn get_packet(self: &Tracker) -> Vec<OscPacket> {
        let rotation = self.rotation.xyz();
        let rotation = [rotation[2], rotation[0], rotation[1]];

        vec![
            OscPacket::Message(OscMessage {
                addr: self.get_position_addr(),
                args: vec![OscType::Array(OscArray {
                    content: self
                        .position
                        .to_array()
                        .iter()
                        .map(|v| OscType::Float(*v))
                        .collect(),
                })],
            }),
            OscPacket::Message(OscMessage {
                addr: self.get_rotation_addr(),
                args: vec![OscType::Array(OscArray {
                    content: rotation.iter().map(|v| OscType::Float(*v)).collect(),
                })],
            }),
        ]
    }
}

impl ToString for Tracker {
    fn to_string(&self) -> String {
        format!("{}|{}|{}", self.name, self.position, self.rotation)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTrackerError;

impl FromStr for Tracker {
    type Err = ParseTrackerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_suffix('\n');
        if s.is_none() {
            return Err(ParseTrackerError);
        }
        let mut split = s.unwrap().split('|');
        let name = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let position = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let rotation = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;

        let name = String::from(name);

        let mut split = position.split(',');
        let x = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let y = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let z = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;

        let x = x.parse::<f32>().map_err(|_| ParseTrackerError)?;
        let y = y.parse::<f32>().map_err(|_| ParseTrackerError)?;
        let z = z.parse::<f32>().map_err(|_| ParseTrackerError)?;

        let position = Vec3::new(x, y, z);

        let mut split = rotation.split(',');
        let x = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let y = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;
        let z = split.next().ok_or(()).map_err(|_| ParseTrackerError)?;

        let x = x.parse::<f32>().map_err(|_| ParseTrackerError)?;
        let y = y.parse::<f32>().map_err(|_| ParseTrackerError)?;
        let z = z.parse::<f32>().map_err(|_| ParseTrackerError)?;

        let rotation = Quat::from_euler(EulerRot::XYZ, x, y, z);

        Ok(Tracker {
            name: name,
            position: position,
            rotation: rotation,
        })
    }
}

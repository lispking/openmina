use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use super::super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2pNetworkYamuxState {
    pub buffer: Vec<u8>,
    pub incoming: VecDeque<YamuxFrame>,
    pub streams: BTreeMap<StreamId, YamuxStreamState>,
    pub terminated: Option<Result<Result<(), YamuxSessionError>, YamuxFrameParseError>>,
    pub init: bool,
}

impl P2pNetworkYamuxState {
    /// Calculates and returns the next available stream ID for outgoing
    /// communication.
    pub fn next_stream_id(&self, client: bool) -> Option<StreamId> {
        // client side should select odd stream IDs
        let suitable_stream_id = move |stream_id: &&StreamId| ((**stream_id & 0x1) == 1) == client;
        if self.init && self.terminated.is_none() {
            let next_stream_id = self
                .streams
                .keys()
                .filter(suitable_stream_id)
                .max()
                .map_or_else(|| if client { 1 } else { 2 }, |stream_id| stream_id + 2);
            Some(next_stream_id)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YamuxStreamState {
    pub incoming: bool,
    pub syn_sent: bool,
    pub established: bool,
    pub readable: bool,
    pub writable: bool,
    pub window_theirs: u32,
    pub window_ours: u32,
}

impl Default for YamuxStreamState {
    fn default() -> Self {
        YamuxStreamState {
            incoming: false,
            syn_sent: false,
            established: false,
            readable: false,
            writable: false,
            window_theirs: 256 * 1024,
            window_ours: 256 * 1024,
        }
    }
}

impl YamuxStreamState {
    pub fn incoming() -> Self {
        YamuxStreamState {
            incoming: true,
            ..Default::default()
        }
    }
}

impl Default for P2pNetworkYamuxState {
    fn default() -> Self {
        P2pNetworkYamuxState {
            buffer: Vec::default(),
            incoming: VecDeque::default(),
            streams: BTreeMap::default(),
            terminated: None,
            init: false,
        }
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct YamuxFlags: u16 {
        const SYN = 0b0001;
        const ACK = 0b0010;
        const FIN = 0b0100;
        const RST = 0b1000;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YamuxPing {
    pub stream_id: StreamId,
    pub opaque: i32,
    pub response: bool,
}

impl YamuxPing {
    pub fn into_frame(self) -> YamuxFrame {
        let YamuxPing {
            stream_id,
            opaque,
            response,
        } = self;
        YamuxFrame {
            flags: if response {
                YamuxFlags::ACK
            } else if stream_id == 0 {
                YamuxFlags::SYN
            } else {
                YamuxFlags::empty()
            },
            stream_id,
            inner: YamuxFrameInner::Ping { opaque },
        }
    }
}

pub type StreamId = u32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum YamuxFrameParseError {
    UnknownVersion(u8),
    UnknownFlags(u16),
    UnknownType(u8),
    UnknownErrorCode(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YamuxFrame {
    pub flags: YamuxFlags,
    pub stream_id: StreamId,
    pub inner: YamuxFrameInner,
}

impl YamuxFrame {
    pub fn into_bytes(self) -> Vec<u8> {
        let data_len = if let YamuxFrameInner::Data(data) = &self.inner {
            data.len()
        } else {
            0
        };
        let mut vec = Vec::with_capacity(12 + data_len);
        vec.push(0);
        match self.inner {
            YamuxFrameInner::Data(data) => {
                vec.push(0);
                vec.extend_from_slice(&self.flags.bits().to_be_bytes());
                vec.extend_from_slice(&self.stream_id.to_be_bytes());
                vec.extend_from_slice(&(data.len() as u32).to_be_bytes());
                vec.extend_from_slice(&data);
            }
            YamuxFrameInner::WindowUpdate { difference } => {
                vec.push(1);
                vec.extend_from_slice(&self.flags.bits().to_be_bytes());
                vec.extend_from_slice(&self.stream_id.to_be_bytes());
                vec.extend_from_slice(&difference.to_be_bytes());
            }
            YamuxFrameInner::Ping { opaque } => {
                vec.push(2);
                vec.extend_from_slice(&self.flags.bits().to_be_bytes());
                vec.extend_from_slice(&self.stream_id.to_be_bytes());
                vec.extend_from_slice(&opaque.to_be_bytes());
            }
            YamuxFrameInner::GoAway(res) => {
                vec.push(3);
                vec.extend_from_slice(&self.flags.bits().to_be_bytes());
                vec.extend_from_slice(&self.stream_id.to_be_bytes());
                let code = match res {
                    Ok(()) => 0u32,
                    Err(YamuxSessionError::Protocol) => 1,
                    Err(YamuxSessionError::Internal) => 2,
                };
                vec.extend_from_slice(&code.to_be_bytes());
            }
        }

        vec
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum YamuxFrameInner {
    Data(Data),
    WindowUpdate { difference: i32 },
    Ping { opaque: i32 },
    GoAway(Result<(), YamuxSessionError>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum YamuxSessionError {
    Protocol,
    Internal,
}
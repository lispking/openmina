use std::net::SocketAddr;

use mina_p2p_messages::rpc_kernel::{QueryHeader, ResponseHeader};
use serde::{Deserialize, Serialize};

use super::{super::*, *};
use crate::{P2pState, PeerId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum P2pNetworkRpcAction {
    Init {
        addr: SocketAddr,
        peer_id: PeerId,
        stream_id: StreamId,
        incoming: bool,
    },
    IncomingData {
        addr: SocketAddr,
        peer_id: PeerId,
        stream_id: StreamId,
        data: Data,
    },
    IncomingMessage {
        addr: SocketAddr,
        peer_id: PeerId,
        stream_id: StreamId,
        message: RpcMessage,
    },
    PrunePending {
        peer_id: PeerId,
        stream_id: StreamId,
    },
    OutgoingQuery {
        peer_id: PeerId,
        query: QueryHeader,
        data: Data,
    },
    OutgoingResponse {
        peer_id: PeerId,
        response: ResponseHeader,
        data: Data,
    },
    OutgoingData {
        addr: SocketAddr,
        peer_id: PeerId,
        stream_id: StreamId,
        data: Data,
        fin: bool,
    },
}

pub enum RpcStreamId {
    Exact(StreamId),
    AnyIncoming,
    AnyOutgoing,
}

impl P2pNetworkRpcAction {
    pub fn stream_id(&self) -> RpcStreamId {
        match self {
            Self::Init { stream_id, .. } => RpcStreamId::Exact(*stream_id),
            Self::IncomingData { stream_id, .. } => RpcStreamId::Exact(*stream_id),
            Self::IncomingMessage { stream_id, .. } => RpcStreamId::Exact(*stream_id),
            Self::PrunePending { stream_id, .. } => RpcStreamId::Exact(*stream_id),
            Self::OutgoingQuery { .. } => RpcStreamId::AnyOutgoing,
            Self::OutgoingResponse { .. } => RpcStreamId::AnyOutgoing,
            Self::OutgoingData { stream_id, .. } => RpcStreamId::Exact(*stream_id),
        }
    }

    pub fn peer_id(&self) -> &PeerId {
        match self {
            Self::Init { peer_id, .. } => peer_id,
            Self::IncomingData { peer_id, .. } => peer_id,
            Self::IncomingMessage { peer_id, .. } => peer_id,
            Self::PrunePending { peer_id, .. } => peer_id,
            Self::OutgoingQuery { peer_id, .. } => peer_id,
            Self::OutgoingResponse { peer_id, .. } => peer_id,
            Self::OutgoingData { peer_id, .. } => peer_id,
        }
    }
}
impl From<P2pNetworkRpcAction> for crate::P2pAction {
    fn from(a: P2pNetworkRpcAction) -> Self {
        Self::Network(a.into())
    }
}

impl redux::EnablingCondition<P2pState> for P2pNetworkRpcAction {
    fn is_enabled(&self, _state: &P2pState, _time: redux::Timestamp) -> bool {
        #[allow(unused_variables)]
        match self {
            P2pNetworkRpcAction::Init {
                addr,
                peer_id,
                stream_id,
                incoming,
            } => true,
            P2pNetworkRpcAction::IncomingData {
                addr,
                peer_id,
                stream_id,
                data,
            } => true,
            P2pNetworkRpcAction::IncomingMessage {
                addr,
                peer_id,
                stream_id,
                message,
            } => true,
            P2pNetworkRpcAction::PrunePending { peer_id, stream_id } => true,
            P2pNetworkRpcAction::OutgoingQuery {
                peer_id,
                query,
                data,
            } => true,
            P2pNetworkRpcAction::OutgoingResponse {
                peer_id,
                response,
                data,
            } => true,
            P2pNetworkRpcAction::OutgoingData {
                addr,
                peer_id,
                stream_id,
                data,
                fin,
            } => true,
        }
    }
}
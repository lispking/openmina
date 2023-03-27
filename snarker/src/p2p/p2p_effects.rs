use crate::rpc::{
    RpcP2pConnectionIncomingAnswerSetAction, RpcP2pConnectionIncomingErrorAction,
    RpcP2pConnectionIncomingSuccessAction, RpcP2pConnectionOutgoingErrorAction,
    RpcP2pConnectionOutgoingSuccessAction,
};
use crate::{Service, Store};

use super::connection::incoming::P2pConnectionIncomingAction;
use super::connection::outgoing::P2pConnectionOutgoingAction;
use super::connection::P2pConnectionAction;
use super::disconnection::P2pDisconnectionAction;
use super::{P2pAction, P2pActionWithMeta};

pub fn p2p_effects<S: Service>(store: &mut Store<S>, action: P2pActionWithMeta) {
    let (action, meta) = action.split();

    match action {
        P2pAction::Connection(action) => match action {
            P2pConnectionAction::Outgoing(action) => match action {
                P2pConnectionOutgoingAction::RandomInit(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::Init(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::Reconnect(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::OfferSdpCreatePending(_) => {}
                P2pConnectionOutgoingAction::OfferSdpCreateSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::OfferReady(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::OfferSendSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::AnswerRecvPending(_) => {}
                P2pConnectionOutgoingAction::AnswerRecvError(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::AnswerRecvSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::FinalizePending(_) => {}
                P2pConnectionOutgoingAction::FinalizeError(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::FinalizeSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::Error(action) => {
                    let p2p = &store.state().p2p;
                    if let Some(rpc_id) = p2p.peer_connection_rpc_id(&action.peer_id) {
                        store.dispatch(RpcP2pConnectionOutgoingErrorAction {
                            rpc_id,
                            error: action.error.clone(),
                        });
                    }
                    // action.effects(&meta, store);
                }
                P2pConnectionOutgoingAction::Success(action) => {
                    let p2p = &store.state().p2p;
                    if let Some(rpc_id) = p2p.peer_connection_rpc_id(&action.peer_id) {
                        store.dispatch(RpcP2pConnectionOutgoingSuccessAction { rpc_id });
                    }
                    action.effects(&meta, store);
                }
            },
            P2pConnectionAction::Incoming(action) => match action {
                P2pConnectionIncomingAction::Init(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::AnswerSdpCreatePending(_) => {}
                P2pConnectionIncomingAction::AnswerSdpCreateSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::AnswerReady(action) => {
                    let p2p = &store.state().p2p;
                    if let Some(rpc_id) = p2p.peer_connection_rpc_id(&action.peer_id) {
                        store.dispatch(RpcP2pConnectionIncomingAnswerSetAction {
                            rpc_id,
                            answer: action.answer.clone(),
                        });
                    }
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::AnswerSendSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::FinalizePending(_) => {}
                P2pConnectionIncomingAction::FinalizeError(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::FinalizeSuccess(action) => {
                    action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::Error(action) => {
                    let p2p = &store.state().p2p;
                    if let Some(rpc_id) = p2p.peer_connection_rpc_id(&action.peer_id) {
                        store.dispatch(RpcP2pConnectionIncomingErrorAction {
                            rpc_id,
                            error: action.error.clone(),
                        });
                    }
                    // action.effects(&meta, store);
                }
                P2pConnectionIncomingAction::Success(action) => {
                    let p2p = &store.state().p2p;
                    if let Some(rpc_id) = p2p.peer_connection_rpc_id(&action.peer_id) {
                        store.dispatch(RpcP2pConnectionIncomingSuccessAction { rpc_id });
                    }
                    action.effects(&meta, store);
                }
            },
        },
        P2pAction::Disconnection(action) => match action {
            P2pDisconnectionAction::Init(action) => action.effects(&meta, store),
            P2pDisconnectionAction::Finish(_action) => {}
        },
        P2pAction::PeerReady(_action) => {
            // action.effects(&meta, store);
        }
    }
}
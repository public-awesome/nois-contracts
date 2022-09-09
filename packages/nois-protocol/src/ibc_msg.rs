use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_slice, to_binary, Binary, Timestamp};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Data;

/// This is the message we send over the IBC channel from nois-proxy to nois-oracle
#[cw_serde]
pub struct RequestBeaconPacket {
    /// Beacon publish time must be > `after`
    pub after: Timestamp,
    /// The address from which the proxy was executed, i.e. the randomness consumer
    pub sender: String,
    pub callback_id: Option<String>,
}

#[cw_serde]
pub enum RequestBeaconPacketAck {
    /// Beacon already exists and this request can be processed immediately.
    Processed {
        /// A RNG specific randomness source identifier, e.g. `drand:<network id>:<round>`
        source_id: String,
    },
    /// Beacon does not yet exist. This request is queued for later.
    Queued {
        /// A RNG specific randomness source identifier, e.g. `drand:<network id>:<round>`
        source_id: String,
    },
}

/// This is the message we send over the IBC channel from nois-oracle to nois-proxy
#[cw_serde]
pub struct DeliverBeaconPacket {
    /// A RNG specific randomness source identifier, e.g. `drand:<network id>:<round>`
    pub source_id: String,
    pub randomness: Data,
    pub sender: String,
    pub callback_id: Option<String>,
}

#[cw_serde]
pub struct DeliverBeaconPacketAck {}

/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// If ibc_receive_packet returns Err(), then x/wasm runtime will rollback the state and return an error message in this format
#[cw_serde]
pub enum StdAck {
    Result(Binary),
    Error(String),
}

impl StdAck {
    // create a serialized success message
    pub fn success(data: impl Serialize) -> Binary {
        let res = to_binary(&data).unwrap();
        StdAck::Result(res).ack()
    }

    // create a serialized error message
    pub fn fail(err: String) -> Binary {
        StdAck::Error(err).ack()
    }

    pub fn ack(&self) -> Binary {
        to_binary(self).unwrap()
    }

    pub fn unwrap(self) -> Binary {
        match self {
            StdAck::Result(data) => data,
            StdAck::Error(err) => panic!("{}", err),
        }
    }

    pub fn unwrap_into<T: DeserializeOwned>(self) -> T {
        from_slice(&self.unwrap()).unwrap()
    }

    pub fn unwrap_err(self) -> String {
        match self {
            StdAck::Result(_) => panic!("not an error"),
            StdAck::Error(err) => err,
        }
    }
}
//! Inlets and outlet request/response types

use std::net::SocketAddr;

use minicbor::{Decode, Encode};
use ockam_core::compat::borrow::Cow;

use ockam_core::CowStr;
#[cfg(feature = "tag")]
use ockam_core::TypeTag;
use ockam_identity::IdentityIdentifier;
use ockam_multiaddr::MultiAddr;
use serde::ser::SerializeStruct;
use serde::Serialize;

/// Request body to create an inlet
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateInlet<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<1407961>,
    /// The address the portal should listen at.
    #[n(1)] listen_addr: SocketAddr,
    /// The peer address.
    /// This can either be the address of an already
    /// created outlet, or a forwarding mechanism via ockam cloud.
    #[n(2)] outlet_addr: MultiAddr,
    /// A human-friendly alias for this portal endpoint
    #[b(3)] alias: Option<CowStr<'a>>,
    /// An authorised identity for secure channels.
    /// Only set for non-project addresses as for projects the project's
    /// authorised identity will be used.
    #[n(4)] authorized: Option<IdentityIdentifier>
}

impl<'a> CreateInlet<'a> {
    pub fn via_project(listen: SocketAddr, to: MultiAddr) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            listen_addr: listen,
            outlet_addr: to,
            alias: None,
            authorized: None,
        }
    }

    pub fn to_node(listen: SocketAddr, to: MultiAddr, auth: Option<IdentityIdentifier>) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            listen_addr: listen,
            outlet_addr: to,
            alias: None,
            authorized: auth,
        }
    }

    pub fn set_alias(&mut self, a: impl Into<Cow<'a, str>>) {
        self.alias = Some(CowStr(a.into()))
    }

    pub fn listen_addr(&self) -> SocketAddr {
        self.listen_addr
    }

    pub fn outlet_addr(&self) -> &MultiAddr {
        &self.outlet_addr
    }

    pub fn authorized(&self) -> Option<IdentityIdentifier> {
        self.authorized.clone()
    }

    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }
}

/// Request body to create an outlet
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct CreateOutlet<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<5351558>,
    /// The address the portal should connect or bind to
    #[b(1)] pub tcp_addr: Cow<'a, str>,
    /// The address the portal should connect or bind to
    #[b(2)] pub worker_addr: Cow<'a, str>,
    /// A human-friendly alias for this portal endpoint
    #[b(3)] pub alias: Option<CowStr<'a>>,
}

impl<'a> CreateOutlet<'a> {
    pub fn new(
        tcp_addr: impl Into<Cow<'a, str>>,
        worker_addr: impl Into<Cow<'a, str>>,
        alias: impl Into<Option<CowStr<'a>>>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: tcp_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
        }
    }
}

/// Response body when interacting with a portal endpoint
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct InletStatus<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<9302588>,
    #[b(1)] pub bind_addr: CowStr<'a>,
    #[b(2)] pub worker_addr: CowStr<'a>,
    #[b(3)] pub alias: CowStr<'a>,
    /// An optional status payload
    #[b(4)] pub payload: Option<CowStr<'a>>,
    #[b(5)] pub outlet_route: CowStr<'a>,
}

impl<'a> Serialize for InletStatus<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("InletStatus", 5)?;
        state.serialize_field("bind_addr", &self.bind_addr)?;
        state.serialize_field("worker_addr", &self.worker_addr)?;
        state.serialize_field("alias", &self.alias)?;
        state.serialize_field("payload", &self.payload)?;
        state.serialize_field("outlet_route", &self.outlet_route)?;
        state.end()
    }
}

impl<'a> InletStatus<'a> {
    pub fn bad_request(reason: &'static str) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            bind_addr: "".into(),
            worker_addr: "".into(),
            alias: "".into(),
            payload: Some(reason.into()),
            outlet_route: "".into(),
        }
    }

    pub fn new(
        bind_addr: impl Into<CowStr<'a>>,
        worker_addr: impl Into<CowStr<'a>>,
        alias: impl Into<CowStr<'a>>,
        payload: impl Into<Option<CowStr<'a>>>,
        outlet_route: impl Into<CowStr<'a>>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            bind_addr: bind_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
            payload: payload.into(),
            outlet_route: outlet_route.into(),
        }
    }
}

/// Response body when interacting with a portal endpoint
#[derive(Clone, Debug, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct OutletStatus<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<4012569>,
    #[b(1)] pub tcp_addr: CowStr<'a>,
    #[b(2)] pub worker_addr: CowStr<'a>,
    #[b(3)] pub alias: CowStr<'a>,
    /// An optional status payload
    #[b(4)] pub payload: Option<CowStr<'a>>,
}

impl<'a> OutletStatus<'a> {
    pub fn bad_request(reason: &'static str) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: "".into(),
            worker_addr: "".into(),
            alias: "".into(),
            payload: Some(reason.into()),
        }
    }

    pub fn new(
        tcp_addr: impl Into<CowStr<'a>>,
        worker_addr: impl Into<CowStr<'a>>,
        alias: impl Into<CowStr<'a>>,
        payload: impl Into<Option<CowStr<'a>>>,
    ) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            tcp_addr: tcp_addr.into(),
            worker_addr: worker_addr.into(),
            alias: alias.into(),
            payload: payload.into(),
        }
    }
}

/// Response body when returning a list of Inlets
#[derive(Debug, Clone, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct InletList<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<8401504>,
    #[b(1)] pub list: Vec<InletStatus<'a>>
}

impl<'a> InletList<'a> {
    pub fn new(list: Vec<InletStatus<'a>>) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            list,
        }
    }
}

/// Response body when returning a list of Outlets
#[derive(Debug, Clone, Decode, Encode)]
#[rustfmt::skip]
#[cbor(map)]
pub struct OutletList<'a> {
    #[cfg(feature = "tag")]
    #[n(0)] tag: TypeTag<8708916>,
    #[b(1)] pub list: Vec<OutletStatus<'a>>
}

impl<'a> OutletList<'a> {
    pub fn new(list: Vec<OutletStatus<'a>>) -> Self {
        Self {
            #[cfg(feature = "tag")]
            tag: TypeTag,
            list,
        }
    }
}

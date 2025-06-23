use std::{fmt::Display, rc::Rc};

use crate::prelude::*;

pub mod prelude {
    pub use super::{Dialogue, DialogueBuilder};
}

mod builder;
mod component;
mod plugin;

impl Dialogue {
    const HEIGHT: u16 = 5;
    const WIDTH: u16 = 100;

    pub fn builder() -> DialogueBuilder {
        DialogueBuilder::default()
    }
    pub fn get_payload(&self) -> Option<DbPayload> {
        self.payload.as_ref().map(|payload| payload.build())
    }
    pub fn get_req_type(&self) -> RequestType {
        self.request_type
    }
    pub fn message_only(message: impl Display) -> Self {
        Self {
            message: message.to_string().into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Dialogue {
    payload: Option<DbPayloadBuilder>,
    message: Rc<str>,
    rect: Rect,
    request_type: RequestType,
}

#[derive(Debug, Default)]
pub struct DialogueBuilder {
    pub payload: Option<DbPayloadBuilder>,
    message: Rc<str>,
    request_type: RequestType,
    pub form_type: Option<AppArm>,
}

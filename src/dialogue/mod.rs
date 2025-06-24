use std::{fmt::Display, rc::Rc};

use crate::prelude::*;

pub mod prelude {
    pub use super::Dialogue;
}

mod builder;
mod component;
mod plugin;
#[cfg(test)]
mod tests;

impl Dialogue {
    const HEIGHT: u16 = 5;
    const WIDTH: u16 = 100;

    pub fn builder() -> DialogueBuilder {
        DialogueBuilder::default()
    }
    pub fn try_get_payload(&self) -> Result<DbPayload> {
        self.payload
            .as_ref()
            .map(|payload| payload.build())
            .ok_or(FormErrors::malformed("payload").into())
    }
    pub fn get_req_type(&self) -> RequestType {
        self.request_type
    }
    pub fn error(message: impl Display) -> Self {
        Self {
            message: message.to_string().into(),
            rect: Rect::new(0, 0, Dialogue::WIDTH, Dialogue::HEIGHT),
            error: true,
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
    error: bool,
}

#[derive(Debug, Default)]
pub struct DialogueBuilder {
    pub payload: Option<DbPayloadBuilder>,
    message: Rc<str>,
    request_type: RequestType,
    pub form_type: Option<AppArm>,
}

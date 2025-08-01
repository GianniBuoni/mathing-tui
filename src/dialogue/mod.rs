use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::prelude::*;

pub mod prelude {
    pub use super::Dialogue;
}

mod builder;
mod component;
mod message;
mod plugin;
#[cfg(test)]
mod tests;

impl Dialogue {
    const HEIGHT: u16 = 4;
    const WIDTH: u16 = 100;

    pub fn builder() -> DialogueBuilder {
        DialogueBuilder::default()
    }
    pub fn try_get_payload(&self) -> Result<DbPayload> {
        self.payload
            .as_ref()
            .map(|payload| payload.build())
            .ok_or(FormError::malformed("payload").into())
    }
    pub fn get_req_type(&self) -> RequestType {
        self.request_type
    }
    pub fn error(message: impl Display) -> Self {
        Self {
            message: Message::new_error(message),
            rect: Rect::new(0, 0, Dialogue::WIDTH, Dialogue::HEIGHT + 1),
            ..Default::default()
        }
    }
    pub fn has_payload(&self) -> bool {
        matches!(self.message, Message::Confirmation(_))
    }
}

#[derive(Debug)]
enum Message {
    Confirmation(Rc<str>),
    Error(Rc<str>),
    Paragraph(Rc<[(Rc<str>, Color)]>),
}

#[derive(Debug, Default)]
pub struct Dialogue {
    payload: Option<DbPayloadBuilder>,
    message: Message,
    rect: Rect,
    request_type: RequestType,
}

#[derive(Debug, Default)]
pub struct DialogueBuilder {
    pub payload: Option<DbPayloadBuilder>,
    message: Vec<(Rc<str>, Color)>,
    request_type: RequestType,
    pub form_type: Option<AppArm>,
}

use super::*;

mod request;

#[derive(Debug, Default)]
pub struct UserParamsBuilder {
    pub u_id: ParamOption<i64>,
    pub name: ParamOption<String>,
}
impl UserParams {
    pub fn builder() -> UserParamsBuilder {
        UserParamsBuilder::default()
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_user_id(mut self, id: i64) -> Self {
        self.u_id = Some(id);
        self
    }
}

impl UserParamsBuilder {
    pub fn with_user_id(&mut self, id: ParamOption<i64>) -> &mut Self {
        self.u_id = id;
        self
    }
    pub fn with_user_name(&mut self, name: ParamOption<String>) -> &mut Self {
        self.name = name;
        self
    }
    pub fn build(&self) -> UserParams {
        UserParams {
            u_id: self.u_id.unwrap(),
            name: self.name.unwrap(),
        }
    }
}

use super::*;

mod request;

impl ItemParams {
    pub fn builder() -> ItemParamsBuilder {
        ItemParamsBuilder::default()
    }
    pub(super) fn new() -> Self {
        Self::default()
    }
    pub(super) fn with_item_id(mut self, item_id: i64) -> Self {
        self.item_id = Some(item_id);
        self
    }
}

#[derive(Debug, Default)]
pub struct ItemParamsBuilder {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub item_id: ParamOption<i64>,
    pub search_filter: ParamOption<String>,
    pub item_name: ParamOption<String>,
    pub item_price: ParamOption<f64>,
}

impl ItemParamsBuilder {
    pub fn with_item_id(&mut self, id: ParamOption<i64>) -> &mut Self {
        self.item_id = id;
        self
    }
    pub fn with_item_name(&mut self, name: ParamOption<String>) -> &mut Self {
        self.item_name = name;
        self
    }
    pub fn with_item_price(&mut self, price: ParamOption<f64>) -> &mut Self {
        self.item_price = price;
        self
    }
    pub fn with_search(
        &mut self,
        search_term: ParamOption<String>,
    ) -> &mut Self {
        self.search_filter = search_term;
        self
    }
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    // TODO: make the build method consuming.
    // righ now the issue is that the builders are owned and
    // muttabed by Form structs
    pub fn build(&self) -> ItemParams {
        ItemParams {
            item_id: self.item_id.unwrap(),
            search_filter: self.search_filter.unwrap(),
            item_name: self.item_name.unwrap(),
            item_price: self.item_price.unwrap(),
            offset: self.offset,
            limit: self.limit,
        }
    }
}

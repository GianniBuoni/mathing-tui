use std::fmt::Display;

use super::*;

impl TableData {
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }
}

impl TableBuilder {
    pub fn with_title(&mut self, title: impl Into<Rc<str>>) -> &mut Self {
        self.title = title.into();
        self
    }
    pub fn with_heading(&mut self, heading: impl Display) -> &mut Self {
        let heading = format!(" {heading} ");
        self.headings.push(heading.into());
        self
    }
    pub fn with_table_type(&mut self, app_arm: AppArm) -> &mut Self {
        self.table_type = Some(app_arm);
        self
    }
    pub fn with_item_limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }
}

impl ComponentBuilder for TableBuilder {
    type Output = TableData;
    fn build(self) -> Result<Self::Output> {
        let Some(table_type) = self.table_type else {
            let message = format!(
                "Malformed table: {} has no defined table type.",
                self.title
            );
            return Err(anyhow::Error::msg(message));
        };

        Ok(TableData {
            title: self.title,
            headings: self.headings.into(),
            table_type: Some(table_type),
            limit: self.limit.unwrap_or(20),
            next_page: 1,
            ..Default::default()
        })
    }
}

use std::fmt::Display;

use super::*;

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
}

impl ComponentBuilder for TableBuilder {
    type Output = TableData;
    fn build(self) -> Self::Output {
        let Some(table_type) = self.table_type else {
            let mut malformed = TableData::default();
            malformed.error =
                Some("Malformed table: built w/o a table type defined.".into());
            return malformed;
        };

        TableData {
            title: self.title,
            headings: self.headings.into(),
            table_type: Some(table_type),
            ..Default::default()
        }
    }
}

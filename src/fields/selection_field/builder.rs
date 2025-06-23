use super::*;

impl<T> SelectionBuilder<T>
where
    T: Debug + Default + Copy,
{
    pub fn with_title(&mut self, title: impl Display) -> &mut Self {
        let title = format!(" {title} ");
        self.title = title.into();
        self
    }
    pub fn with_multselect(&mut self) -> &mut Self {
        self.multiselect = true;
        self
    }
}

impl<T> ComponentBuilder for SelectionBuilder<T>
where
    T: Debug + Default + Copy,
{
    type Output = SelectionField<T>;

    fn build(mut self) -> Result<Self::Output> {
        self.choices
            .iter_mut()
            .enumerate()
            .for_each(|(index, f)| f.init(index, self.active_choice.clone()));
        Ok(Self::Output {
            choices: self.choices,
            active_choice: self.active_choice,
            title: self.title,
            multiselect: self.multiselect,
            ..Default::default()
        })
    }
}

impl<T> PluginParent for SelectionBuilder<T> where T: Debug + Default + Copy {}

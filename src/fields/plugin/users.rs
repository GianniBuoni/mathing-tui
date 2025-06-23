use super::*;

pub fn new_user_inputs(parent: &mut FormBuilder) -> Result<()> {
    let params = try_get_user_params(parent)?;

    let mut name_input = InputField::<String>::new();
    name_input
        .with_title("User Name")
        .with_field_type(AppArm::Users);
    let name = name_input.value.clone();

    params.user_name(name);
    name_input.plugin(parent)?;

    Ok(())
}

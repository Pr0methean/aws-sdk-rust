// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_skill_authorization_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_skill_authorization::DeleteSkillAuthorizationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.skill_id {
        object.key("SkillId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.room_arn {
        object.key("RoomArn").string(var_2.as_str());
    }
    Ok(())
}

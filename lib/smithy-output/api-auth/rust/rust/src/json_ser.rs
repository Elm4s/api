// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_complete_email_verification_input(
	object: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::input::CompleteEmailVerificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	if let Some(var_1) = &input.code {
		object.key("code").string(var_1.as_str());
	}
	if let Some(var_2) = &input.verification_id {
		object.key("verification_id").string(var_2.as_str());
	}
	Ok(())
}

pub fn serialize_structure_crate_input_refresh_identity_token_input(
	object: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::input::RefreshIdentityTokenInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	if let Some(var_3) = &input.logout {
		object.key("logout").boolean(*var_3);
	}
	Ok(())
}

pub fn serialize_structure_crate_input_start_email_verification_input(
	object: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::input::StartEmailVerificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	if let Some(var_4) = &input.captcha {
		let mut object_5 = object.key("captcha").start_object();
		crate::json_ser::serialize_union_crate_model_captcha_config(&mut object_5, var_4)?;
		object_5.finish();
	}
	if let Some(var_6) = &input.email {
		object.key("email").string(var_6.as_str());
	}
	if let Some(var_7) = &input.game_id {
		object.key("game_id").string(var_7.as_str());
	}
	Ok(())
}

pub fn serialize_union_crate_model_captcha_config(
	object_5: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::model::CaptchaConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	match input {
		crate::model::CaptchaConfig::Hcaptcha(inner) => {
			let mut object_8 = object_5.key("hcaptcha").start_object();
			crate::json_ser::serialize_structure_crate_model_captcha_config_hcaptcha(
				&mut object_8,
				inner,
			)?;
			object_8.finish();
		}
		crate::model::CaptchaConfig::Turnstile(inner) => {
			let mut object_9 = object_5.key("turnstile").start_object();
			crate::json_ser::serialize_structure_crate_model_captcha_config_turnstile(
				&mut object_9,
				inner,
			)?;
			object_9.finish();
		}
		crate::model::CaptchaConfig::Unknown => {
			return Err(
				aws_smithy_http::operation::SerializationError::unknown_variant("CaptchaConfig"),
			)
		}
	}
	Ok(())
}

pub fn serialize_structure_crate_model_captcha_config_hcaptcha(
	object: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::model::CaptchaConfigHcaptcha,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	if let Some(var_10) = &input.client_response {
		object.key("client_response").string(var_10.as_str());
	}
	Ok(())
}

pub fn serialize_structure_crate_model_captcha_config_turnstile(
	object: &mut aws_smithy_json::serialize::JsonObjectWriter,
	input: &crate::model::CaptchaConfigTurnstile,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
	if let Some(var_11) = &input.client_response {
		object.key("client_response").string(var_11.as_str());
	}
	Ok(())
}

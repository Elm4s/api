// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_verify_custom_hostname_verify_custom_hostname_output_body(
	body: &[u8],
) -> std::result::Result<
	std::option::Option<std::string::String>,
	crate::error::VerifyCustomHostnameError,
> {
	(!body.is_empty())
		.then(|| {
			let body_str = std::str::from_utf8(body)
				.map_err(crate::error::VerifyCustomHostnameError::unhandled)?;
			Ok(body_str.to_string())
		})
		.transpose()
}

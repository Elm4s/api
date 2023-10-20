use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;

#[operation(name = "chat-message-validate")]
async fn handle(
	ctx: OperationContext<chat_message::validate::Request>,
) -> GlobalResult<chat_message::validate::Response> {
	let message = unwrap_ref!(ctx.message);
	let topic = unwrap_ref!(message.topic);
	let topic_kind = unwrap_ref!(topic.kind);
	let body = unwrap_ref!(message.body);
	let body_kind = unwrap_ref!(body.kind);

	tokio::try_join!(
		validate(&ctx, body_kind, topic_kind),
		// Validate message body
		op!([ctx] chat_message_body_validate {
			body: Some(body.clone()),
		}),
	)?;

	Ok(chat_message::validate::Response {})
}

// TODO: Validate that team join messages only go to team topics, etc
async fn validate(
	ctx: &OperationContext<chat_message::validate::Request>,
	body_kind: &backend::chat::message_body::Kind,
	topic_kind: &backend::chat::topic::Kind,
) -> GlobalResult<()> {
	Ok(())
}

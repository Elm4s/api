use proto::backend::{self, pkg::*};
use rivet_operation::prelude::*;

#[derive(Debug, sqlx::FromRow)]
struct ChatMessage {
	message_id: Uuid,
	send_ts: i64,
	body: Vec<u8>,
}

#[operation(name = "chat-message-list")]
async fn handle(
	ctx: OperationContext<chat_message::list::Request>,
) -> GlobalResult<chat_message::list::Response> {
	let crdb = ctx.crdb().await?;

	let thread_id = unwrap_ref!(ctx.thread_id).as_uuid();
	let direction = unwrap!(chat_message::list::request::QueryDirection::from_i32(
		ctx.query_direction
	));

	let messages = match direction {
		chat_message::list::request::QueryDirection::Before => {
			let mut msgs = sql_fetch_all!(
				[ctx, ChatMessage]
				"
				SELECT message_id, send_ts, body
				FROM db_chat.messages
				WHERE thread_id = $1 AND send_ts < $2
				ORDER BY send_ts DESC, message_id DESC
				LIMIT $3
				",
				thread_id,
				ctx.ts,
				ctx.count as i64,
			)
			.await?;
			msgs.reverse();
			msgs
		}
		chat_message::list::request::QueryDirection::BeforeAndAfter => {
			sql_fetch_all!(
				[ctx, ChatMessage]
				"
				SELECT * FROM (
					SELECT message_id, send_ts, body
					FROM db_chat.messages
					WHERE thread_id = $1 AND send_ts <= $2
					ORDER BY send_ts DESC, message_id DESC
					LIMIT $3
				)

				UNION

				SELECT * FROM (
					SELECT message_id, send_ts, body
					FROM db_chat.messages
					WHERE thread_id = $1 AND send_ts > $2
					ORDER BY send_ts ASC, message_id ASC
					LIMIT $3
				)

				ORDER BY send_ts ASC, message_id ASC
				",
				thread_id,
				ctx.ts,
				ctx.count as i64,
			)
			.await?
		}
		chat_message::list::request::QueryDirection::After => {
			sql_fetch_all!(
				[ctx, ChatMessage]
				"
				SELECT message_id, send_ts, body
				FROM db_chat.messages
				WHERE thread_id = $1 AND send_ts > $2
				ORDER BY send_ts ASC, message_id ASC
				LIMIT $3
				",
				thread_id,
				ctx.ts,
				ctx.count as i64,
			)
			.await?
		}
	};

	let messages = messages
		.into_iter()
		.map(|message| {
			let body = backend::chat::MessageBody::decode(message.body.as_slice())?;

			Ok(backend::chat::Message {
				chat_message_id: Some(message.message_id.into()),
				thread_id: Some(thread_id.into()),
				send_ts: message.send_ts,
				body: Some(body),
			})
		})
		.collect::<GlobalResult<Vec<backend::chat::Message>>>()?;

	Ok(chat_message::list::Response { messages })
}

use serenity::model::channel::{Message, MessageType};

struct DiscordMessage(Message);

impl bote::User for DiscordMessage {
	fn nick(&self) -> &str {
		self.0
			.member
			.as_ref()
			.and_then(|member| member.nick.as_ref())
			.unwrap_or(&self.0.author.name)
	}

	fn user(&self) -> &str {
		&self.0.author.name
	}
}

impl bote::Sender for DiscordMessage {
	type User = Self;

	fn sender(&self) -> &Self::User {
		&self
	}
}

impl bote::Text for DiscordMessage {
	fn content(&self) -> &str {
		&self.0.content
	}
}

impl bote::Message for DiscordMessage {
	type Text = DiscordMessage;
	type Join = DiscordMessage;
	type Leave = DiscordMessage;
	type Other = DiscordMessage;

	fn sender(&self) -> Option<&str> {
		Some(&self.0.author.name)
	}

	fn visit<T: bote::Visitor<Self>>(&self, visitor: &T) {
		// TODO: How to know when user left?
		match self.0.kind {
			MessageType::Regular => visitor.visit_text(&self),
			MessageType::MemberJoin => visitor.visit_join(&self),
			_ => visitor.visit_other(&self),
		}
	}
}

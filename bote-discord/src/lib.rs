use serenity::model::channel::{Message, MessageType};

struct DiscordMessage(Message);

impl bote::Text for DiscordMessage {
	fn sender(&self) -> &str {
		&self.0.author.name
	}

	fn content(&self) -> &str {
		&self.0.content
	}
}

impl bote::Message for DiscordMessage {
	type Text = DiscordMessage;
	type Other = DiscordMessage;

	fn sender(&self) -> Option<&str> {
		Some(&self.0.author.name)
	}

	fn visit<T: bote::Visitor<Self>>(&self, visitor: &T) {
		match self.0.kind {
			MessageType::Regular => visitor.visit_text(&self),
			_ => visitor.visit_other(&self),
		}
	}
}

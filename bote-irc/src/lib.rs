use irc::client::prelude::{Command, Message};

struct IrcText<'a> {
	message: &'a Message,
	content: &'a String,
}

impl<'a> bote::Text for IrcText<'a> {
	fn sender(&self) -> &str {
		self.message.source_nickname().expect("Text")
	}

	fn content(&self) -> &str {
		self.content
	}
}

struct IrcMessage<'a> {
	message: &'a Message,
}

impl<'a> bote::Message for IrcMessage<'a> {
	type Text = IrcText<'a>;
	type Other = Self;

	fn sender(&self) -> Option<&str> {
		self.message.source_nickname()
	}

	fn visit<T: bote::Visitor<Self>>(&self, visitor: &T) {
		match self.message.command {
			Command::PRIVMSG(_, ref content) => {
				let text = IrcText {
					message: self.message,
					content,
				};
				visitor.visit_text(&text);
			}
			_ => {
				visitor.visit_other(self);
			}
		}
	}
}

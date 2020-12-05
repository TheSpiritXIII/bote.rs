use irc::client::prelude::{Command, Message};

struct IrcText<'a> {
	message: &'a Message,
	content: &'a String,
}

impl<'a> bote::User for IrcText<'a> {
	fn nick(&self) -> &str {
		self.message.source_nickname().unwrap()
	}

	fn user(&self) -> &str {
		// TODO: Must lookup user by nick.
		self.nick()
	}
}

impl<'a> bote::Sender for IrcText<'a> {
	type User = Self;

	fn sender(&self) -> &Self::User {
		self
	}
}

impl<'a> bote::Text for IrcText<'a> {
	fn content(&self) -> &str {
		self.content
	}
}

struct IrcMessage<'a> {
	message: &'a Message,
}

// TODO: Merge with IrcText.
struct IrcSender<'a> {
	message: &'a Message,
}

impl<'a> bote::User for IrcSender<'a> {
	fn user(&self) -> &str {
		self.message.source_nickname().unwrap()
	}
}

impl<'a> bote::Sender for IrcSender<'a> {
	type User = Self;

	fn sender(&self) -> &Self::User {
		self
	}
}

impl<'a> bote::Message for IrcMessage<'a> {
	type Text = IrcText<'a>;
	type Join = IrcSender<'a>;
	type Leave = IrcSender<'a>;
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
			Command::JOIN(..) => visitor.visit_join(&IrcSender {
				message: self.message,
			}),
			Command::PART(..) => visitor.visit_leave(&IrcSender {
				message: self.message,
			}),
			_ => {
				visitor.visit_other(self);
			}
		}
	}
}

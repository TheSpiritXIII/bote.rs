use std::cell::RefCell;

/// A user of the chat.
pub trait User {
	/// The nick name of the user.
	fn nick(&self) -> &str {
		self.user()
	}

	/// The user name of the user.
	fn user(&self) -> &str;
}

/// A message type with a sender.
pub trait Sender {
	type User: User;

	/// The sender of the message.
	fn sender(&self) -> &Self::User;
}

/// Represents a plain text message.
pub trait Text: Sender {
	/// The value of the text.
	fn content(&self) -> &str;
}

/// Visitor pattern for accessing a message by its type.
pub trait Visitor<T: Message + ?Sized> {
	/// Visit a [`Text`](Text) message type.
	fn visit_text(&self, _text: &T::Text) {}

	/// Visit a join message type.
	fn visit_join(&self, _text: &T::Join) {}

	/// Visit a leave message type.
	fn visit_leave(&self, _text: &T::Leave) {}

	/// Visit some other unknown message type.
	fn visit_other(&self, _other: &T::Other) {}
}

/// A message from a chat server.
pub trait Message {
	type Text: Text;
	type Join: Sender;
	type Leave: Sender;
	type Other;

	/// The sender of the message, if the message has one.
	fn sender(&self) -> Option<&str>;

	/// Visits the message type.
	fn visit<T: Visitor<Self>>(&self, visitor: &T);
}

/// A bot plugin for processing messages.
pub trait Plugin<T: Message> {
	/// Runs the plugin with the given message. Returns true if this plugin stops proceeding plugins
	/// from running.
	fn run(&mut self, message: &T) -> bool;
}

/// Runs the chain of plugins for the given message.
pub fn run<T: Message>(plugin_list: &[Box<RefCell<dyn Plugin<T>>>], message: &T) {
	plugin_list
		.iter()
		.any(|plugin| plugin.borrow_mut().run(message));
}

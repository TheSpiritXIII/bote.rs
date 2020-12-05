use std::cell::RefCell;

/// Represents a plain text message.
pub trait Text {
	/// The sender of the message.
	fn sender(&self) -> &str;

	/// The value of the text.
	fn content(&self) -> &str;
}

/// Visitor pattern for accessing a message by its type.
pub trait Visitor<T: Message + ?Sized> {
	/// Visit a [`Text`](Text) message type.
	fn visit_text(&self, text: &T::Text);

	/// Visit some other unknown message type.
	fn visit_other(&self, other: &T::Other);
}

/// A message from a chat server.
pub trait Message {
	type Text: Text;
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

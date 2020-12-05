# bote-rs
A write once run everywhere of bot frameworks by bridging different chat clients other.

## Usage
Add `bote` to your `Cargo.toml`:
```toml
[dependencies]
bote = 0.1
```

At the base of it all is the `Plugin` trait. Implementations of this trait operate on `Message` which represents a server event. If we want to write a plugin which simply outputs all received plaintext message to the terminal, it would look as follows:
```rust
struct LogPlugin;

impl<T: Message> Visitor<T> for LogPlugin {
	fn visit_text(&self, text: &T::Text) {
		println!("<{}> {}", text.sender().nick(), text.content());
	}
}

impl<T: Message> Plugin<T> for LogPlugin {
	fn run(&mut self, message: &T) -> bool {
		message.visit(self);
		false
	}
}
```

You might want one chat bot that rules them all and consists of multiple plugins. A utility method is provided for this:
```rust
let plugin_list = vec![RefCell::new(LogPlugin), RefCell::new(EchoPlugin)]
bote::run(&plugin_list, message);
```

Note, that this utility is very bare-bones. It doesn't have a concept of which plugin must run before which other plugin, so you must take care of that yourself. Some plugins in the list may tell exit the execution of proceeding plugins (see `Plugin::run`).

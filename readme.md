## RapidSockets Rust SDK

### Introduction
This is the official Software Development Kit for Rust to interact with the RapidSockets real-time messaging platform.

### Installation
```
cargo install rapidsockets
```

### Usage
```rust
let mut config = rapidsockets::Config::init();
config.set_key("your key");

let mut rs = rapidsockets::Client::connect(config);

fn user_demo(packet: String) {
    println!("{}", packet);
}

let mut subscription = rapidsockets::Subscription::init(&rs);
subscription.set_channel("user_demo".to_string());
subscription.set_callback(user_demo);
subscription.subscribe();

let mut message = rapidsockets::Message::init(&rs);
message.set_channel("user_demo".to_string());
message.set_message("my_message".to_string());
message.publish();
```

### Development specific notes
```
# run
cargo run
```

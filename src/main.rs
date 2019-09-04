pub mod rapidsockets {

    pub type Callback = fn(String);
    pub fn noop(_: String) {}

    pub struct Client {
        pub gateway: &'static str,
        pub api: &'static str,
        //pub connection: WebSocket,
        pub authenticated: bool,
        pub session: String,
        pub packet_queue: Vec<String>,
        pub key: String
    }

    impl Client {

        pub fn init() -> Client {
            Client {
                gateway: "wss://gateway.rapidsockets.com",
                api: "https://api.rapidsockets.com",
                authenticated: false,
                session: "".to_string(),
                packet_queue: Vec::new(),
                key: "".to_string()
            }
        }

        pub fn set_key(&mut self, key: &'static str) {
            self.key = key.to_string();
        }

    }

    pub struct Subscription<'a> {
        pub client: &'a Client,
        pub channel: String,
        pub callback: Callback
    }

    impl<'a> Subscription<'a> {

        pub fn init(client: &Client) -> Subscription {
            Subscription {
                client: client,
                channel: "".to_string(),
                callback: noop
            }
        }

        pub fn set_channel(&mut self, channel: String) {
            self.channel = channel;
        }

        pub fn set_callback(&mut self, callback: Callback) {
            self.callback = callback;
        }

    }

    pub struct Message<'a> {
        pub client: &'a Client,
        pub channel: String,
        pub message: String,
    }

    impl<'a> Message<'a> {

        pub fn init(client: &Client) -> Message {
            Message {
                client: client,
                channel: "".to_string(),
                message: "".to_string()
            }
        }

        pub fn set_channel(&mut self, channel: String) {
            self.channel = channel;
        }

        pub fn set_message(&mut self, message: String) {
            self.message = message;
        }

    }

}



fn main() {
    let mut rs = rapidsockets::Client::init();
    rs.set_key("mul-f75b9b5c-7b50-47ac-b937-c1909242d0ce");

    fn user_demo(packet: String) {
        println!("{}", packet);
    }

    let mut subscription = rapidsockets::Subscription::init(&rs);
    subscription.set_channel("user_demo".to_string());
    subscription.set_callback(user_demo);

    let mut message = rapidsockets::Message::init(&rs);
    message.set_channel("user_demo".to_string());
    message.set_message("my_message".to_string());
}

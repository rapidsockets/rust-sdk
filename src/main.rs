extern crate websocket;
extern crate tokio;

mod rapidsockets {

    use websocket::ClientBuilder;
    use websocket::Message as WsMessage;
    use websocket::client::sync::Client as WsClient;
    use websocket::stream::sync::NetworkStream;

    fn noop(_: String) {}

    pub struct Config {
        gateway: String,
        api: String,
        key: String
    }

    impl Config {

        pub fn init() -> Config {
            Config {
                gateway: "wss://gateway.rapidsockets.com".to_string(),
                api: "https://api.rapidsockets.com".to_string(),
                key: "".to_string()
            }
        }

        pub fn set_key(&mut self, key: &'static str) {
            self.key = key.to_string();
        }

        pub fn set_gateway(&mut self, gateway: &'static str) {
            self.gateway = gateway.to_string();
        }

        pub fn set_api(&mut self, api: &'static str) {
            self.api = api.to_string();
        }

    }

    pub struct Client<'a> {
        connection: WsClient<Box<NetworkStream + Send>>,
        authenticated: bool,
        session: String,
        packet_queue: Vec<String>,
        cbs: Vec<fn()>,
        subscriptions: Vec<Subscription<'a>>
    }

    impl<'a> Client<'a> {

        pub fn connect(config: Config) -> Client<'a> {
            Client {
                connection: ClientBuilder::new(&config.gateway)
                    .unwrap()
                    .connect(None)
                    .unwrap(),
                authenticated: false,
                session: "".to_string(),
                packet_queue: Vec::new(),
                cbs: Vec::new(),
                subscriptions: Vec::new()
            }
        }

        pub fn test(&mut self) {
            let message = WsMessage::text("test");
            self.connection.send_message(&message).unwrap();
        }

    }

    pub struct Subscription<'b> {
        client: &'b Client<'b>,
        channel: String,
        callback: fn(String)
    }

    impl<'b> Subscription<'b> {

        pub fn init(client: &'b Client<'b>) -> Subscription<'b> {
            Subscription {
                client: client,
                channel: "".to_string(),
                callback: noop
            }
        }

        pub fn subscribe(&self) {

        }

        pub fn set_channel(&mut self, channel: String) {
            self.channel = channel;
        }

        pub fn set_callback(&mut self, callback: fn(String)) {
            self.callback = callback;
        }

    }

    pub struct Message<'c> {
        pub client: &'c Client<'c>,
        pub channel: String,
        pub message: String,
    }

    impl<'c> Message<'c> {

        pub fn init(client: &'c Client<'c>) -> Message<'c> {
            Message {
                client: client,
                channel: "".to_string(),
                message: "".to_string()
            }
        }

        pub fn publish(&self) {

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
    let mut config = rapidsockets::Config::init();
    config.set_gateway("ws://127.0.0.1:2007");
    config.set_api("http://127.0.0.1:2008");
    config.set_key("mul-f75b9b5c-7b50-47ac-b937-c1909242d0ce");

    let mut rs = rapidsockets::Client::connect(config);
    rs.test();

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
}

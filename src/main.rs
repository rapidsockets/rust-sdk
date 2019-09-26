extern crate json;
extern crate ws;

mod rapidsockets {

    use json;
    use ws::{connect, CloseCode};

    fn noop(_: String) {}

    pub struct Config {
        gateway: String,
        api: String,
        key: String,
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
        config: Config,
        authenticated: bool,
        session: String,
        packet_queue: Vec<String>,
        cbs: Vec<fn()>,
        subscriptions: Vec<Subscription<'a>>,
    }

    impl<'a> Client<'a> {

        pub fn init(config: Config) -> Client<'a> {
            Client {
                config: config,
                authenticated: false,
                session: "".to_string(),
                packet_queue: Vec::new(),
                cbs: Vec::new(),
                subscriptions: Vec::new()
            }
        }

        pub fn connect(&mut self) {
            let packet =
                json::object!{
                    "action" => "authorize",
                    "payload" => json::object!{
                        "key" => self.config.key.to_string()
                    }
                };

            connect(self.config.gateway.to_string(), |out| {
                out.send(packet.dump()).unwrap();

                move |msg| {
                    println!("Got message: {}", msg);
                    Ok(())
                }
            }).unwrap();
        }

    }

    pub struct Subscription<'a> {
        client: &'a Client<'a>,
        channel: String,
        callback: fn(String),
    }

    impl<'a> Subscription<'a> {

        pub fn init(client: &'a Client<'a>) -> Subscription<'a> {
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

    pub struct Message<'a> {
        pub client: &'a Client<'a>,
        pub channel: String,
        pub message: String,
    }

    impl<'a> Message<'a> {

        pub fn init(client: &'a Client<'a>) -> Message<'a> {
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

    let mut rs = rapidsockets::Client::init(config);

    fn user_demo(packet: String) {
        println!("{}", packet);
    }

    let mut subscription = rapidsockets::Subscription::init(&rs);
    subscription.set_channel("user_demo".to_string());
    subscription.set_callback(user_demo);
    subscription.subscribe();

    rs.connect();

    let mut message = rapidsockets::Message::init(&rs);
    message.set_channel("user_demo".to_string());
    message.set_message("my_message".to_string());
    message.publish();
}

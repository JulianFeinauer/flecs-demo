extern crate core;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use flecs_api::apis::configuration::Configuration;
use flecs_api::apis::default_api::system_ping_get;
use structopt::StructOpt;
use futures_util::stream::StreamExt;
use futures_util::{FutureExt};

use paho_mqtt as mqtt;
use paho_mqtt::Message;

// Tests
mod test;
mod commands;

fn main() {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let cli = Arc::new(Mutex::new(mqtt::AsyncClient::new("tcp://mqtt.eclipseprojects.io:1883").unwrap()));

    let connect_options = mqtt::ConnectOptionsBuilder::default()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    let client = cli.clone();
    let rt2 = rt.clone();
    cli.lock().unwrap().set_connected_callback(move |_| {
        println!("Connected, starting subscriptions...");
        let token = client.lock().unwrap().subscribe("test", 0).map(|_|{
            println!("Subscribed!");
        });
        rt2.spawn(token);
        println!("Sent subscription request...")
    });
    cli.lock().unwrap().set_message_callback(|_, m| {
        match m {
            Some(m) => {
                println!("Message: {}", m);
            }
            _ => {}
        }
    });

    let client = cli.clone();
    let rt2 = rt.clone();
    thread::spawn(move || {
        println!("Hello from sender thread");

        let mut count = 0;

        loop {
            println!("Sending Message {count}...");
            rt2.spawn(client.lock().unwrap().publish(Message::new("test", format!("Hallo {count}"), 2)).map(|x| {
                match x {
                    Ok(_) => {
                        println!("Message sent!")
                    }
                    Err(_) => {
                        println!("Unable to send message!")
                    }
                };
            }));
            sleep(Duration::from_secs(2));
            count += 1;
        }
    });

    // Connect MQTT
    let client = cli.clone();
    rt.spawn(client.lock().unwrap().connect(connect_options).map(|_|{
        println!("Connection established!");
    }));

    // Block on the receive loop
    rt.block_on(async {
        let mut stream = client.lock().unwrap().get_stream(25);

        while let Some(msg) = stream.next().await {
            if let Some(msg) = msg {
                println!("Message: {}", msg);
            } else {
                println!("We are disconnected...");
            }
        }
    });
}

#[allow(dead_code)]
fn main2() {
    #[derive(StructOpt)]
    #[structopt(name = "session_example")]
    struct Opt {
        #[structopt(short = "h", long, default_value = "http://127.0.0.1:8951")]
        host: String,
    }

    let opt = Opt::from_args();
    let conf = Configuration {
        // base_path: "http://10.20.40.165:8951".to_string(),
        base_path: opt.host,
        user_agent: None,
        client: Default::default(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    };
    loop {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = rt.block_on(system_ping_get(&conf));

        match result {
            Ok(result) => {
                println!("Able to ping the system on {}", &conf.base_path);
                match result.additional_info {
                    None => {}
                    Some(s) => {
                        println!("Response: {}", s);
                    }
                }
            }
            Err(_) => {
                println!("Unable to ping the system on {}", &conf.base_path);
            }
        }

        sleep(Duration::from_secs(5));
    }
}

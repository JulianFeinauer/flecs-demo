extern crate core;

use flecs_api::apis::configuration::Configuration;
use flecs_api::apis::default_api::system_ping_get;
use futures_util::{pin_mut, select, stream::StreamExt, FutureExt};
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

use paho_mqtt as mqtt;
use paho_mqtt::Message;
use tokio::runtime::Handle;

// Tests
mod commands;
mod test;

#[tokio::main]
async fn main() {
    let cli = mqtt::AsyncClient::new("tcp://mqtt.eclipseprojects.io:1883").unwrap();

    let connect_options = mqtt::ConnectOptionsBuilder::default()
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(10))
        .finalize();

    let mut cli = cli;
    let mut stream = cli.get_stream(25).fuse();

    let rt = Handle::current();
    cli.connect_with_callbacks(
        connect_options,
        move |client, _| {
            println!("Connected, starting subscriptions...");
            // we need to spawn here, as the callback is not async
            let client = client.clone();
            rt.spawn(async move {
                client
                    .subscribe("test", 0)
                    .map(|_| {
                        println!("Subscribed!");
                    })
                    .await
            });
            println!("Sent subscription request...")
        },
        |_, _, _| {},
    )
    .await
    .unwrap();

    let sender = async {
        println!("Hello from sender");

        let mut count = 0;
        loop {
            match cli
                .publish(Message::new("test", format!("Hallo {count}"), 2))
                .await
            {
                Ok(_) => {
                    println!("Message sent!")
                }
                Err(err) => {
                    println!("Unable to send message!: {err}")
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            count += 1;
        }
    }
    .fuse();
    pin_mut!(sender);

    loop {
        select! {
             msg = stream.next() => match msg {
                Some(Some(msg)) => {
                    println!("Message: {msg}");
                }
                _ => {
                    println!("We are disconnected...");
                    break;
                }
            },
            _ = sender => {}
        }
    }
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

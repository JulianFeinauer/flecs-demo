use std::thread::sleep;
use std::time::Duration;
use flecs_api::apis::configuration::Configuration;
use flecs_api::apis::default_api::system_ping_get;
use structopt::StructOpt;

// Tests
mod test;

fn main() {
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

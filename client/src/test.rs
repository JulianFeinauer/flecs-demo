#[cfg(test)]
mod test {
    use flecs_api::apis::configuration::Configuration;
    use flecs_api::apis::default_api::{app_install_post, app_list_get, system_ping_get};
    use flecs_api::apis::Error;
    use flecs_api::models::{AppInstallPostRequest};

    #[test]
    fn test_call() {
        let conf = Configuration {
            base_path: "http://10.20.40.165:8951".to_string(),
            user_agent: None,
            client: Default::default(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(app_list_get(&conf));
        match result {
            Ok(inner) => {
                match inner.additional_info {
                    Some(s) => println!("Additional Info: {}", s),
                    _ => {}
                };
                match inner.app_list {
                    None => {}
                    Some(list) => {
                        for app in list.iter() {
                            match &app.app {
                                Some(name) => println!("App: {}", name),
                                _ => continue
                            }
                        }
                    }
                }
            }
            Err(e) => {
                panic!("Unable to fetch URL {}", e);
            }
        }

        let result = rt.block_on(app_install_post(&conf, AppInstallPostRequest {
            app: Some(String::from("org.apache.iotdb")),
            license_key: None,
            version: Some(String::from("0.13.0-node"))
        }));

        match result {
            Ok(_) => {
                println!("Was installed!")
            }
            Err(e) => {
                println!("Was not installed :(");
                match e {
                    Error::Reqwest(_) => {}
                    Error::Serde(_) => {}
                    Error::Io(_) => {}
                    Error::ResponseError(e) => {
                        println!("Status was {}", e.status);
                        println!("Message: {}", e.content);
                    }
                }
            }
        }

        let result = rt.block_on(system_ping_get(&conf));

        match result {
            Ok(result) => {
                println!("Able to ping the system");
                match result.additional_info {
                    None => {}
                    Some(s) => {
                        println!("Response: {}", s);
                    }
                }
            }
            Err(_) => {
                println!("Unable to ping the system...")
            }
        }
    }
}

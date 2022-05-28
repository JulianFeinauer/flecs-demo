#[cfg(test)]
mod test {
    use flecs_api::apis::configuration::Configuration;
    use flecs_api::apis::default_api::{app_install_post, app_list_get};
    use flecs_api::models::AppInstallPostRequest;

    #[test]
    fn test_call() {
        let conf = Configuration {
            base_path: "http://localhost:8080".to_string(),
            user_agent: None,
            client: Default::default(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        };
        let mut rt = tokio::runtime::Runtime::new().unwrap();
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
            app: Some(String::from("iotdb")),
            license_key: Some(String::from("my-license")),
            version: Some(String::from("0.1.0"))
        }));
    }
}

use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    assert_eq!(1, 1);
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() -> String {
    //actix_webserver::run().await
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    println!("runnin server. address: http://127.0.0.1:{}", port);

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
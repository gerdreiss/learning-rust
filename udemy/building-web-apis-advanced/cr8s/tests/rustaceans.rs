use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK)
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Jane Doe",
            "email": "jane@doe.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let json: Value = response.json().unwrap();
    assert_eq!(json["name"], "Jane Doe");
    assert_eq!(json["email"], "jane@doe.com");
    assert!(json["id"].is_i64());
    assert!(json["created_at"].is_string());
}

#[test]
fn test_get_rustacean() {
    let client = Client::new();
    let json: Value = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&json!({
            "name": "Jane Doe",
            "email": "jane@doe.com"
        }))
        .send()
        .unwrap()
        .json()
        .unwrap();

    let id = json["id"].as_i64().unwrap();

    let response = client
        .get(format!("http://127.0.0.1:8000/rustaceans/{}", id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert_eq!(json["name"], "Jane Doe");
    assert_eq!(json["email"], "jane@doe.com");
    assert!(json["id"].is_i64());
    assert!(json["created_at"].is_string());
}

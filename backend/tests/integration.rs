//! Integration tests hit the live backend. Run against a docker-compose stack:
//!   docker compose up -d postgres backend
//!   cargo test -- --ignored
//!
//! By default only the pure unit-style test below runs in CI (no DB required).

#[test]
fn smoke_arithmetic() {
    // Sanity placeholder so `cargo test` always has something to run.
    assert_eq!(2 + 2, 4);
}

#[tokio::test]
#[ignore]
async fn health_endpoint_returns_ok() {
    let base = std::env::var("BACKEND_BASE").unwrap_or_else(|_| "http://localhost:3001".into());
    let resp = reqwest::get(format!("{base}/health"))
        .await
        .expect("request");
    assert!(resp.status().is_success());
    let json: serde_json::Value = resp.json().await.expect("json");
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
#[ignore]
async fn notes_crud_roundtrip() {
    let base = std::env::var("BACKEND_BASE").unwrap_or_else(|_| "http://localhost:3001".into());
    let client = reqwest::Client::new();

    let created: serde_json::Value = client
        .post(format!("{base}/api/notes"))
        .json(&serde_json::json!({ "text": "hello from tests" }))
        .send()
        .await
        .expect("post")
        .error_for_status()
        .expect("2xx")
        .json()
        .await
        .expect("json");
    let id = created["id"].as_str().expect("id").to_string();

    let list: serde_json::Value = client
        .get(format!("{base}/api/notes"))
        .send()
        .await
        .expect("get")
        .json()
        .await
        .expect("json");
    assert!(list.as_array().expect("array").iter().any(|n| n["id"] == id));

    let del = client
        .delete(format!("{base}/api/notes/{id}"))
        .send()
        .await
        .expect("delete");
    assert_eq!(del.status(), 204);
}

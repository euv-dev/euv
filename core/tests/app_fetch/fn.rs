use super::*;

#[wasm_bindgen_test]
fn a_response_reports_whether_the_server_accepted_the_request() {
    let ok: FetchResponse = FetchResponse::new(200, b"{}".to_vec());
    assert!(ok.is_ok());
    let missing: FetchResponse = FetchResponse::new(404, Vec::new());
    assert!(!missing.is_ok());
    let failed: FetchResponse = FetchResponse::new(500, Vec::new());
    assert!(!failed.is_ok());
}

#[wasm_bindgen_test]
fn a_response_keeps_the_status_and_the_body_the_server_sent() {
    let response: FetchResponse = FetchResponse::new(200, b"{\"count\":2}".to_vec());
    assert_eq!(response.status(), 200);
    assert_eq!(response.bytes(), b"{\"count\":2}".to_vec());
    assert_eq!(response.text(), Some("{\"count\":2}".to_string()));
}

#[wasm_bindgen_test]
fn a_body_that_is_not_utf8_has_no_text() {
    let response: FetchResponse = FetchResponse::new(200, vec![0xff, 0xfe, 0xfd]);
    assert_eq!(response.text(), None);
    assert_eq!(response.bytes().len(), 3);
}

#[wasm_bindgen_test]
fn the_request_methods_carry_their_wire_names() {
    assert_eq!(FetchMethod::Get.as_str(), "GET");
    assert_eq!(FetchMethod::Post.as_str(), "POST");
    assert_eq!(FetchMethod::Put.as_str(), "PUT");
    assert_eq!(FetchMethod::Delete.as_str(), "DELETE");
}

#[wasm_bindgen_test]
fn a_failure_names_the_stage_it_stopped_at() {
    let transport: FetchError = FetchError::Transport("network".to_string());
    assert!(transport.to_string().contains("transport"));
    let body: FetchError = FetchError::Body("truncated".to_string());
    assert!(body.to_string().contains("body"));
    assert_ne!(transport, body);
}

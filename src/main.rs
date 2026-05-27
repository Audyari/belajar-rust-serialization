// buat helo word

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct UserLoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct UserLoginResponse {
    token: String,
    role: String,
}

fn main() {
    let login_data = UserLoginRequest {
        username: "admin".to_string(),
        password: "secret123".to_string(),
    };

    let request_json = serde_json::to_string(&login_data).unwrap();
    println!("📤 Request ke server: {}", request_json);

    let received: UserLoginRequest = serde_json::from_str(&request_json).unwrap();

    if received.username == "admin" && received.password == "secret123" {
        // Server buat response
        let response = UserLoginResponse {
            token: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string(),
            role: "admin".to_string(),
        };

        // Server serialize response ke JSON
        let response_json = serde_json::to_string(&response).unwrap();
        println!("📥 Response dari server: {}", response_json);
        // Output: {"token":"eyJhbGci...","role":"admin"}

        // ================================================================
        // BAGIAN 3: CLIENT RECEIVE RESPONSE
        // ================================================================
        // Client terima JSON, deserialize
        let login_response: UserLoginResponse = serde_json::from_str(&response_json).unwrap();

        println!("\n✅ LOGIN BERHASIL!");
        println!("   Token: {}...", &login_response.token[..20]);
        println!("   Role: {}", login_response.role);

        // Token bisa disimpan untuk request selanjutnya
        // std::env::set_var("AUTH_TOKEN", login_response.token);
    } else {
        println!("❌ Login gagal!");
    }
}

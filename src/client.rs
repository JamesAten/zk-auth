use num_bigint::BigUint;
use std::io::stdin;

pub mod zk_auth {
    include!("zkp_auth.rs");
}

use zk_auth::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    RegisterRequest,
};
use zk::ZKP;

#[tokio::main]
async fn main() {
    let mut buf = String::new();
    let (alpha, beta, p, q) = ZKP::get_constants();
    let zkp = ZKP {
        alpha: alpha.clone(),
        beta: beta.clone(),
        p: p.clone(),
        q: q.clone(),
    };

    let mut client = AuthClient::connect("http://127.0.0.1:50051")
        .await
        .expect("unable to connect to the server");
    println!("Successfully connected to the server");

    println!("Enter Username:");
    stdin()
        .read_line(&mut buf)
        .expect("Could not process the username");
    let username = buf.trim().to_string();
    buf.clear();

    println!("Enter your password");
    stdin()
        .read_line(&mut buf)
        .expect("Could not process the username");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let (y1, y2) = zkp.compute_pair(&password);

    let request = RegisterRequest {
        user: username.clone(),
        y1: y1.to_bytes_be(),
        y2: y2.to_bytes_be(),
    };

    let _response = client
        .register(request)
        .await
        .expect("Unable to register");

    println!("Registration Successful");

    println!("Enter password for Login:");
    stdin()
        .read_line(&mut buf)
        .expect("Could not process the username");
    let password = BigUint::from_bytes_be(buf.trim().as_bytes());
    buf.clear();

    let k = ZKP::generate_random_number_below(&q);
    let (r1, r2) = zkp.compute_pair(&k);

    let request = AuthenticationChallengeRequest {
        user: username,
        r1: r1.to_bytes_be(),
        r2: r2.to_bytes_be(),
    };

    let response = client
        .create_authentication_challenge(request)
        .await
        .expect("Request Challenge to the Server Failed")
        .into_inner();

    let auth_id = response.auth_id;
    let c = BigUint::from_bytes_be(&response.c);

    let s = zkp.solve(&k, &c, &password);

    let request = AuthenticationAnswerRequest {
        auth_id,
        s: s.to_bytes_be(),
    };

    let response = client
        .verify_authentication(request)
        .await
        .expect("Unable to Authenticate with the Server")
        .into_inner();

    println!("Login successful! session_id: {}", response.session_id);
}

use std::net::TcpListener;
use zero2prod::run; // Replace 'my_project' with the actual namespace where the run function is located.

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setup logging if not already set up (optional)
    std::env::set_var("RUST_LOG", "actix_web=info");

    // Bind to an address with a specific port or use port 0 if you prefer a random available port
    let listener = TcpListener::bind("127.0.0.1:0") // You can also use port 0 here for a random port in production
        .expect("Failed to bind to port");

    // Extract the port which might be useful for logging or other purposes
    let port = listener.local_addr().unwrap().port();
    println!("Running on port {}", port);

    // Call the run function, passing the listener
    let server = run(listener).expect("Failed to start the server");

    // Await the server which will run until stopped
    server.await
}

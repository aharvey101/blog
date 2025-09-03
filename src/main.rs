mod blog;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rust Static Blog Generator");
    println!("============================");
    
    // Generate the static site
    blog::generate_site()?;
    
    // Start the development server
    println!("\nStarting development server...");
    let address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    server::start_server(&address)?;
    
    Ok(())
}

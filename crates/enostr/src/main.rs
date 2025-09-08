use enostr::FullKeypair;

fn main() {
    let keypair = FullKeypair::generate();
    println!("Generated Keypair:");
    println!(
        "  Public Key: {}",
        keypair
            .pubkey
            .npub()
            .unwrap_or_else(|| keypair.pubkey.hex())
    );
    // In a real application, you would handle the secret key securely.
    // For this example, we'll just acknowledge its existence.
    println!("  Secret Key: <hidden>");
}

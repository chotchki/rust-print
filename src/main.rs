use rust_print::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    nodescope_cli::run().await
}

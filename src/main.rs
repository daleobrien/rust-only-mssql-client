use std::env;
use tiberius::{Client, Config, AuthMethod, EncryptionLevel};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env_or("SQL_HOST", "localhost");
    let port: u16 = env_or("SQL_PORT", "1433").parse().unwrap_or(1433);
    let user = env_or("SQL_USER", "sa");
    let password = env_or("SQL_PASSWORD", "YourStrongPassword123!");
    let database = env_or("SQL_DATABASE", "master");

    // Build a configuration for the SQL Server connection.
    let mut config = Config::new();

    config.host(&host);
    config.port(port);
    config.authentication(AuthMethod::sql_server(&user, &password));
    config.database(&database);
    config.encryption(EncryptionLevel::Required);
    config.trust_cert(); // needed for self-signed / dev certificates

    // Connect via TCP.
    let addr = format!("{host}:{port}");
    let tcp = TcpStream::connect(&addr).await?;
    tcp.set_nodelay(true)?;

    // tiberius uses tokio 0.2-era AsyncRead/AsyncWrite; bridge with compat.
    let mut client = Client::connect(config, tcp.compat_write()).await?;

    println!("✅ Connected to SQL Server at {host}:{port}.\n");

    // -------------------------------------------------------
    // 1. Query server version
    // -------------------------------------------------------
    let query = "SELECT @@VERSION AS version;";
    let stream = client.query(query, &[]).await?;
    let rows = stream.into_results().await?;

    for row in rows.first().into_iter().flatten() {
        let version: Option<&str> = row.get("version");
        if let Some(v) = version {
            println!("📦 Server version:\n{v}\n");
        }
    }

    // -------------------------------------------------------
    // 2. Create a sample table and insert / query data
    // -------------------------------------------------------
    client
        .simple_query(
            "
            IF OBJECT_ID('dbo.greetings', 'U') IS NOT NULL
                DROP TABLE dbo.greetings;

            CREATE TABLE dbo.greetings (
                id   INT IDENTITY(1,1) PRIMARY KEY,
                msg  NVARCHAR(100) NOT NULL
            );

            INSERT INTO dbo.greetings (msg) VALUES
                (N'Hello from Rust! 🦀'),
                (N'Bonjour depuis Rust!'),
                (N'Hola desde Rust!');
            ",
        )
        .await?;

    println!("📋 Created and populated 'greetings' table.\n");

    // -------------------------------------------------------
    // 3. Query the sample table
    // -------------------------------------------------------
    let stream = client
        .query("SELECT id, msg FROM dbo.greetings ORDER BY id;", &[])
        .await?;
    let rows = stream.into_results().await?;

    println!("📋 Results from 'greetings':");
    for row in rows.first().into_iter().flatten() {
        let id: Option<i32> = row.get("id");
        let msg: Option<&str> = row.get("msg");
        match (id, msg) {
            (Some(id), Some(msg)) => println!("  [{id}] {msg}"),
            _ => {}
        }
    }

    println!("\n🎉 Done.");
    Ok(())
}

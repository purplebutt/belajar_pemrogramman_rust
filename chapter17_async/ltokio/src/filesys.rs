use tokio::io::{self, AsyncWriteExt};


async fn write_file(target: &str, text: &str) -> io::Result<()> {
    let mut f = tokio::fs::File::create(target).await?;
    f.write(text.as_bytes()).await?;
    f.flush().await?;
    Ok(())
}

fn read_file(target: &str) -> io::Result<String> {
    std::fs::read_to_string(target)
}

pub async fn test() -> io::Result<()> {
    let filename = "test.txt";
    write_file(filename, "Sample text").await?;

    let contents = tokio::task::spawn_blocking(
        || read_file(filename)
    ).await??;

    println!("File contents: {}", contents);

    tokio::fs::remove_file(filename).await?;

    Ok(())
}

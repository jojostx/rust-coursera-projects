use std::error::Error;
use std::io::{Read, Write};

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::io;
use std::net::SocketAddr;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};

const SERVER: Token = Token(0);
fn main() -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let mut poll: Poll = Poll::new()?;
    let mut events: Events = Events::with_capacity(1024);

    let mut listener = TcpListener::bind(addr)?;
    println!("{}", red("now accepting connections"));

    let mut clients: Vec<(Token, Arc<Mutex<TcpStream>>)> = vec![];
    let mut next_token_id = 1;

    // Start listening for incoming connections.
    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in &events {
            if event.token() == SERVER && event.is_readable() {
                let (mut client_stream, _) = listener.accept()?;
                let token = Token(next_token_id);
                next_token_id += 1;

                poll.registry()
                    .register(&mut client_stream, token, Interest::READABLE)?;

                clients.push((token, Arc::new(Mutex::new(client_stream))));
                continue;
            }

            let mut disconnected: Vec<usize> = vec![];

            let mut messages = Vec::new();
            let mut buf = [0u8; 1024];
            let mut buffer = Vec::new(); // grows over time

            for (idx, (token, client)) in clients.iter().enumerate() {
                if event.token() == *token && event.is_readable() {
                    let client_stream_ = Arc::clone(client);
                    let mut client_stream = client_stream_.lock().unwrap();

                    match client_stream.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            buffer.extend_from_slice(&buf[..n]);

                            while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
                                let line = buffer.drain(..=pos).collect::<Vec<u8>>();
                                let msg = String::from_utf8_lossy(&line);
                                println!("Received: {}", msg.trim_end());
                                messages.push(msg.trim_end().to_string());
                            }

                            let message = messages.join(" ");

                            // Successfully read a message
                            for (_, other_client) in &clients {
                                let mut client_stream_other = other_client.lock().unwrap();
                                if !compare_tcp_streams(&client_stream, &client_stream_other) {
                                    client_stream_other.write_all(message.as_bytes()).ok();
                                }
                            }
                        }
                        Ok(0) => {
                            // EOF reached (client disconnected)
                            // Close client_socket
                            close_tcp_stream(&client_stream)?;
                            // Remove client_socket from clients list
                            disconnected.push(idx);
                        }
                        _ => (),
                    }
                }
            }

            // remove from clients here
            for idx in disconnected.into_iter().rev() {
                clients.remove(idx);
            }
        }
    }
}

fn compare_tcp_streams(stream1: &TcpStream, stream2: &TcpStream) -> bool {
    #[cfg(unix)]
    {
        // Unix-based system: Compare raw file descriptors
        stream1.as_raw_fd() != stream2.as_raw_fd()
    }

    #[cfg(windows)]
    {
        // Windows system: Compare raw socket handles
        stream1.as_raw_socket() != stream2.as_raw_socket()
    }
}

fn close_tcp_stream(stream: &TcpStream) -> io::Result<()> {
    stream.shutdown(std::net::Shutdown::Both) // Shutdown both read and write
}

fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

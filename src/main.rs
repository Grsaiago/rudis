use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const READ_SIZE: usize = 1024;

#[derive(Debug)]
struct RedisClient {
    stream: BufReader<TcpStream>,
    read_buffer: String,
    write_buffer: String,
}

impl RedisClient {
    fn from_stream(stream: TcpStream) -> Self {
        RedisClient {
            stream: BufReader::new(stream),
            read_buffer: String::default(),
            write_buffer: String::default(),
        }
    }

    async fn parse_query(query: &[u8]) {
        println!(
            "O length do comando enviado foi: [{}] e o comando é {}",
            query.len(),
            std::str::from_utf8(query).unwrap()
        );
    }
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379")
        .await
        .expect(&format!("Error binding on port {}", 6379));
    println!("Redis server listening on 127.0.0.1:6379");

    loop {
        match listener.accept().await {
            Ok((new_client, _)) => {
                tokio::spawn(async move {
                    let mut client = RedisClient::from_stream(new_client);
                    println!("New client!");
                    // verificar se no buffer há algum comando completo
                    // caso tenha, executar o comando e dar consume no len de
                    // bytes lidos. (https://docs.rs/tokio/latest/tokio/io/struct.BufReader.html#method.fill_buf)
                    // terminar a leitura
                    // esse read_to_string tá dando hang pq o socket só
                    // manda EOF quando fecha.
                    loop {
                        let buff = client
                            .stream
                            .fill_buf()
                            .await
                            .expect("Error reading from socket");

                        if buff.is_empty() {
                            // pique control D da minishell
                            println!("Connection closed, closing client...");
                            return;
                        }

                        RedisClient::parse_query(buff).await;

                        let size = buff.len();

                        client
                            .stream
                            .write(b"+PONG\r\n")
                            .await
                            .expect("Error writting to client");

                        client.stream.consume(size);
                    }
                    /*
                    match client.stream.read_to_string(&mut client.read_buffer).await {
                        Ok(val) => println!("Valor lido do read_to_string = {}", val),
                        Err(e) => println!("error: {}", e),
                    };
                    println!("Passou do read_to_string");
                    for _ in client.read_buffer.lines() {
                        if let Err(e) = client.stream.write(b"+PONG\r\n").await {
                            println!("error: {}", e);
                        }
                    }
                    */
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}

/*
#[cfg(test)]
mod test {
    use std::{sync::mpsc, thread};

    use super::*;

    #[test]
    fn ping() {
        thread::scope(|s| {
            let (sender, receiver) = mpsc::channel::<()>();
            s.spawn(move || {
                let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
                let _ = sender.send(());
                let (new_client, _) = listener.accept().unwrap();
                let mut client = RedisClient::from_stream(new_client);
                client
                    .stream
                    .set_nonblocking(true)
                    .expect("Error on nonblocking");
                let _ = client.stream.read_to_string(&mut client.read_buffer);
                for _ in client.read_buffer.lines() {
                    client
                        .stream
                        .write(b"+PONG\r\n")
                        .expect("Error writting to client");
                }
                let _ = sender.send(());
            });

            s.spawn(move || {
                let mut buff: [u8; 14] = [0; 14];
                let _ = receiver.recv();
                let mut client = TcpStream::connect("127.0.0.1:6379").unwrap();
                let _ = client.write(b"PING\nPING");
                let _ = receiver.recv();
                let _ = client.read(&mut buff);
                assert_eq!(buff, *b"+PONG\r\n+PONG\r\n");
            });
        });
    }
}
*/

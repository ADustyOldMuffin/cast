use tokio::{net::UdpSocket, io};

struct Message {
    
}

struct CastServer {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl CastServer {
    async fn run(self) -> Result<(), io::Error> {
        let CastServer {
            socket,
            mut buf,
        } = self;

        loop {
            let msg_result = socket.recv_from(&mut buf).await?;
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

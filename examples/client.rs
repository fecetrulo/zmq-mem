use std::ops::Deref;
use zmq::{self, SocketType};
static BYTES: usize = 1_000_000;
static META1: &str = "meta1";
static META2: &str = "meta2";

fn main() {
    let mut count = 0;
    {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(SocketType::REQ).unwrap();
        let result = {
            socket.connect("ipc:///tmp/server.ipc").unwrap();

            socket.send_multipart(&["GET_INFO"], 0).unwrap();
            let status_msg = socket.recv_msg(0).unwrap();
            match status_msg.deref() {
                b"OK" => {
                    let mut result = Vec::new();
                    let mut meta1 = zmq::Message::new();
                    let mut meta2 = zmq::Message::new();
                    let mut info = zmq::Message::new();

                    while socket.get_rcvmore().unwrap() {
                        socket.recv(&mut meta1, 0).unwrap();
                        socket.recv(&mut meta2, 0).unwrap();
                        socket.recv(&mut info, 0).unwrap();

                        result.push((meta1.to_vec(), meta2.to_vec(), info.to_vec()))
                    }
                    result
                }
                _ => panic!("unexpected response"),
            }
        };

        // Checks if the messages are correct
        result.into_iter().for_each(|(meta1, meta2, info)| {
            if std::str::from_utf8(&meta1).unwrap() == META1
                && std::str::from_utf8(&meta2).unwrap() == META2
                && info.len() == BYTES
            {
                count += 1;
            }
        });
    }

    println!("message count: {}", count);
    loop {}
}

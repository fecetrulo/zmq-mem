use zmq;
static BYTES: usize = 1_000_000;
static META1: &str = "meta1";
static META2: &str = "meta2";

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SocketType::ROUTER).unwrap();
    socket.bind("ipc:///tmp/server.ipc").unwrap();
    let events = socket.get_events().unwrap() as zmq::PollEvents;
    let mut poll_itens = Vec::new();
    poll_itens.push(socket.as_poll_item(events));
    loop {
        zmq::poll(&mut poll_itens, -1);
        let identity_msg = socket.recv_msg(0).unwrap();
        let _separator_msg = socket.recv_msg(0).unwrap();
        let request_type = socket.recv_msg(0).unwrap();
    
        match request_type.as_ref() {
            b"GET_INFO" => {
                socket.send(identity_msg, zmq::SNDMORE).unwrap();
                socket.send("", zmq::SNDMORE).unwrap();
                send_info(&socket);
            }
            x => panic!("wrong call {}", std::str::from_utf8(x).unwrap()),
        }
    }
}

pub fn send_info(socket: &zmq::Socket) {
    socket.send("OK", zmq::SNDMORE).unwrap();

    let info: Vec<u8> = vec![0; BYTES];
    let meta1 = META1;
    let meta2 = META2;
    let qtd = 5;

    (1..=qtd).for_each(|i| {
        socket.send(meta1, zmq::SNDMORE).unwrap();
        socket.send(meta2, zmq::SNDMORE).unwrap();
        if i == qtd {
            socket.send(&info, 0).unwrap();
        } else {
            socket.send(&info, zmq::SNDMORE).unwrap();
        };
    });
    println!("done sending");
}

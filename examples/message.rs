use zmq;
static BYTES: usize = 1_000_000;
static META: &str = "meta";

fn main() {
    let qtd = 2000;
    
    let info: Vec<u8> = vec![0; BYTES];
    let meta = META;
    let mut result = Vec::new();

    (0..qtd).for_each(|_| {
        let meta = zmq::Message::from(meta);
        let info = zmq::Message::from(&info);
        result.push((meta.to_vec(), info.to_vec()))
    });
    
    let mut count = 0;
    result.into_iter().for_each(|(meta, info)| {
        if std::str::from_utf8(&meta).unwrap() == META
            && info.len() == BYTES
        {
            count += 1;
        }
    });

    println!("count {}", count);
}

use crate::osc::tracker::Tracker;
use rosc::encoder;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread::spawn;
use std::time::Duration;

pub struct Manager {
    trackers: HashMap<String, Tracker>,
    send_sock: UdpSocket,
    recv_sock: UdpSocket,
}

impl Manager {
    pub fn new(send_ip: Option<SocketAddr>, recv_ip: Option<SocketAddr>) -> Manager {
        let send_sock = UdpSocket::bind("0.0.0.0:0").unwrap();
        let recv_sock = UdpSocket::bind("0.0.0.0:0").unwrap();

        match send_ip {
            Some(addr) => send_sock.connect(addr),
            None => Ok(()),
        };
        match recv_ip {
            Some(addr) => recv_sock.connect(addr),
            None => Ok(()),
        };

        Manager {
            trackers: HashMap::new(),
            send_sock: send_sock,
            recv_sock: recv_sock,
        }
    }

    pub fn send(self: &Manager) {
        let bundles = self.trackers.values().map(|t| t.get_packet());

        for bundle in bundles {
            for packet in bundle {
                let msg_buf = encoder::encode(&packet).unwrap();

                self.send_sock.send(&msg_buf).unwrap();
            }
        }
    }

    pub fn main_loop(mut self: Manager) -> (mpsc::Sender<String>, mpsc::Receiver<String>) {
        let (_osc_in_tx, osc_in_rx) = mpsc::channel::<String>();
        let (osc_out_tx, osc_out_rx) = mpsc::channel::<String>();

        spawn(move || loop {
            self.send();

            let data = osc_out_rx.recv_timeout(Duration::from_millis(50));

            match data {
                Ok(d) => {
                    let tracker = Tracker::from_str(d.as_str());
                    if tracker.is_ok() {
                        let tracker = tracker.unwrap();
                        println!("Received Tracker: {}", tracker.to_string());

                        self.trackers
                            .entry(tracker.name.to_string())
                            .and_modify(|t| {
                                t.position = tracker.position;
                                t.rotation = tracker.rotation;
                            })
                            .or_insert(tracker);
                    }
                }
                Err(_) => {}
            }
        });

        return (osc_out_tx, osc_in_rx);
    }
}

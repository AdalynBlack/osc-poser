pub mod osc;

use crate::osc::manager::Manager;
use crate::osc::tracker::Tracker;
use ctrlc;
use interprocess::os::unix::fifo_file::create_fifo;
use std::fs::{read_to_string, remove_file};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const FIFO_PATH: &str = "/tmp/osc-poser";

fn main() {
    let (sig_tx, sig_rx) = channel();
    ctrlc::set_handler(move || sig_tx.send(()).unwrap());

    let send_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000);
    let recv_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9001);
    let manager = Manager::new(Some(send_addr), Some(recv_addr));

    let (osc_out, _osc_in) = manager.main_loop();

    let left_foot = Tracker::new("1");
    let right_foot = Tracker::new("2");

    osc_out.send(left_foot.to_string());
    osc_out.send(right_foot.to_string());

    let fifo_rx = setup_fifo();

    let mut time = SystemTime::now();

    while sig_rx.try_recv().is_err() {
        let _deltaTime = time.elapsed().unwrap();
        time = SystemTime::now();

        let received = fifo_rx.try_recv();
        if received.is_ok() {
            let received = received.unwrap();
            osc_out.send(received);
        }

        let elapsed = time.elapsed().unwrap();
        if elapsed < Duration::new(0, 10000000) {
            sleep(Duration::new(0, 10000000) - elapsed);
        }
    }

    remove_file(FIFO_PATH);
}

fn setup_fifo() -> Receiver<String> {
    create_fifo(FIFO_PATH, 0o777).expect("Another process is already running!");

    let (tx, rx) = channel::<String>();

    thread::spawn(move || loop {
        let data = read_to_string(FIFO_PATH).unwrap();
        tx.send(data).unwrap();
    });
    rx
}

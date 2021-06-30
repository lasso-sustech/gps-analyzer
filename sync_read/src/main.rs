use std::{thread, time};
use std::sync::{mpsc,};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
//
use rppal::gpio::Gpio;
use serialport;

const IO1:u8 = 23;
const IO2:u8 = 12;

fn gps_recv(tx : mpsc::Sender<String>) {
    let timer = time::Instant::now();
    let mut serial_buf: Vec<u8> = vec![0; 1024];

    let ports = serialport::available_ports().expect("No ports found!");
    let port = ports[0].port_name.clone();
    let mut port = serialport::new(port, 115_200)
            .timeout(time::Duration::from_millis(1000))
            .open().expect("Failed to open port");
    
    loop {
        if let Ok(_) = port.read( serial_buf.as_mut_slice() ) {
            let msg = String::from_utf8( serial_buf.clone() ).unwrap();
            tx.send(msg).unwrap();
        }
        else {
            println!("[{:.6}] Timeout from GPS thread.", timer.elapsed().as_secs_f32());
        }
    }
}

fn file_writer(rx : mpsc::Receiver<String>) {
    let timer = time::Instant::now();
    let count = Path::new("./logs/").iter().count();
    let file_name = format!("gps-{}.log", count);
    let mut file = File::create(file_name).unwrap();
    let timeout = time::Duration::from_millis(1000);

    loop {
        if let Ok(msg) = rx.recv_timeout(timeout) {
            file.write_all( msg.as_bytes() ).unwrap();
            file.sync_data().unwrap();
        }
        else {
            println!("[{:.6}] Timeout from GPS thread.", timer.elapsed().as_secs_f32());
        }
    }
}

fn main() {
    // init GPIO setup
    let gpio = Gpio::new().unwrap();
    let pin1 = gpio.get(IO1).unwrap().into_input();
    let pin2 = gpio.get(IO2).unwrap().into_input();

    // spawn gps thread
    let (gps_tx, gps_rx) = mpsc::channel::<String>();
    let gps_handle = thread::spawn(move || {
        gps_recv(gps_tx);
    });

    //spawn writer thread
    let (w_tx, w_rx) = mpsc::channel::<String>();
    let writer_handle = thread::spawn(move || {
        file_writer(w_rx);
    });

    // main timer loop
    let time_step = time::Duration::from_millis(1); //1ms
    loop {
        thread::sleep(time_step);
        let io1 = pin1.read() as u8;
        let io2 = pin2.read() as u8;
        let gps_msg = {
            if let Ok(msg) = gps_rx.try_recv() {
                msg
            } else {
                String::new()
            }
        };
        let datum = format!("{} | {} | {}", io1, io2, gps_msg);
        w_tx.send(datum).unwrap();
        break;
    }

    //cleanup
    gps_handle.join().unwrap();
    writer_handle.join().unwrap();
}

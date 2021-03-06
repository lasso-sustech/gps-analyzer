use std::{thread, time};
use std::sync::{mpsc,};
use std::fs::File;
use std::io::prelude::*;
//
use glob::glob;
use rppal::gpio::Gpio;
use serialport;

const IO1:u8 = 27;
const IO2:u8 = 23;
const PACE:u64 = 1; //1 millisecond

fn gps_recv(tx : mpsc::Sender<String>) {
    let timer = time::Instant::now();
    let mut serial_buf: Vec<u8> = vec![0; 256];
    let mut buf: Vec<u8> = vec![0; 1024];

    // let ports = serialport::available_ports().expect("No ports found!");
    // let port = ports[0].port_name.clone();
    let mut port = serialport::new("/dev/ttyUSB0", 38_400)
            .timeout(time::Duration::from_millis(1000))
            .open().expect("Failed to open port");
    
    loop {
        if let Ok(size) = port.read( serial_buf.as_mut_slice() ) {
            buf.extend( serial_buf[..size].iter().copied() );
            if let Some(pos) = buf.iter().position( |&x| x == '\n' as u8 ) {
                let msg:Vec<u8> = buf.drain(..pos).collect();
                buf.remove(0); //remove remaining '\n'
                if let Ok(mut msg) = String::from_utf8(msg) {
                    msg.retain( |c| !c.is_ascii_control() );
                    tx.send(msg).unwrap();
                }
            }
        }
        else {
            println!("[{:.6}] Timeout from GPS thread.", timer.elapsed().as_secs_f32());
        }
    }
}

fn file_writer(rx : mpsc::Receiver<String>) {
    let timer = time::Instant::now();
    let timeout = time::Duration::from_millis(1000);

    // create logs folder
    std::fs::create_dir("logs").unwrap_or(());
    let count = glob("logs/*.log").unwrap().count();
    println!("{}", count);
    // open file
    let file_name = format!("logs/gps-{:02}.log", count);
    let mut file = File::create(file_name).unwrap();

    loop {
        if let Ok(msg) = rx.recv_timeout(timeout) {
            writeln!(file, "{}", msg).unwrap_or(());
            // file.write_all( msg.as_bytes() ).unwrap();
            // file.write_all( b"\n" ).unwrap();
            // file.sync_data().unwrap();
        }
        else {
            println!("[{:.6}] Timeout from file-writer thread.", timer.elapsed().as_secs_f32());
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
    let time_step = time::Duration::from_millis(PACE);
    let spin_sleeper = spin_sleep::SpinSleeper::new(100_000);
    loop {
        spin_sleeper.sleep(time_step);
        // thread::sleep(time_step);
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
        if let Err(_) = w_tx.send(datum) {
            break;
        }
    }

    //cleanup
    gps_handle.join().unwrap();
    writer_handle.join().unwrap();
}

#[test]
fn gpio_read() {
    let gpio = Gpio::new().unwrap();
    let pin1 = gpio.get(IO1).unwrap().into_input();
    let pin2 = gpio.get(IO2).unwrap().into_input();

    let time_step = time::Duration::from_millis(1000);
    loop {
        thread::sleep(time_step);
        let io1 = pin1.read() as u8;
        let io2 = pin2.read() as u8;
        println!("pin-{}: {}; pin-{}: {}", IO1, io1, IO2, io2);
    }
}

#[test]
fn gps_read() {
    let mut serial_buf: Vec<u8> = vec![0; 1024];

    let ports = serialport::available_ports().expect("No ports found!");
    let port = ports[0].port_name.clone();
    let mut port = serialport::new(port, 38_400)
            .timeout(time::Duration::from_millis(1000))
            .open().expect("Failed to open port");
    
    loop {
        if let Ok(_) = port.read( serial_buf.as_mut_slice() ) {
            let msg = String::from_utf8( serial_buf.clone() ).unwrap();
            println!("{}", msg);
        }
    }
}

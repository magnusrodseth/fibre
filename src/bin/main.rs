use std::{
    fs,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use fibre::ThreadPool;

fn main() {
    run();
}

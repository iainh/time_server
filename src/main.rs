/**
 * Copyright (c) 2017 Iain H
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

fn get_time() -> String {
    let now = SystemTime::now();
    let seconds = now.duration_since(UNIX_EPOCH).unwrap();
    seconds.as_secs().to_string()
}

fn main() {
    let addr = "0.0.0.0:9000".parse().unwrap();
    let mut core = Core::new().unwrap();

    let listener = TcpListener::bind(&addr, &core.handle()).unwrap();

    let clients = listener.incoming();
    let times = clients.and_then(|(socket, _peer_addr)| {
        tokio_io::io::write_all(socket, get_time())
    });

    let server = times.for_each(|(_socket, _time)| {
        Ok(())
    });

    core.run(server).unwrap();
}

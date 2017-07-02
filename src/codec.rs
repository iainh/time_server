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
use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct TimeCodec;

impl Codec for TimeCodec {
    type In = u64;
    type Out = u64;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {

        if let Some(_) = buf.as_ref().iter().position(|b| *b == b'\n') {
            let length = buf.len();
            let line = buf.drain_to(length);
            // Turn this data into a UTF-8 string and return it
            return match str::from_utf8(line.as_ref()) {
                       Ok(_) => Ok(Some(5)),
                       Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
                   };
        }
        Ok(None)
    }

    fn encode(&mut self, _: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        let now = SystemTime::now();
        let seconds = now.duration_since(UNIX_EPOCH).unwrap();
        let time = seconds.as_secs().to_string();

        for byte in time.as_bytes() {
            buf.push(*byte);
        }
        Ok(())
    }
}
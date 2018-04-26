use std::net::TcpStream;
use std::io::{self, Write};
use common::Note;
use byteorder::{WriteBytesExt, BigEndian};

pub struct MidiClient {
    connection: TcpStream
}

impl MidiClient {
    /// Create a new MidiClient
    pub fn new(address: &str) -> io::Result<MidiClient> {
        Ok(
            MidiClient {
                connection: TcpStream::connect(address)?
            }
        )
    }

    /// Send a note and its start time to the MIDI server
    pub fn send(&mut self, note: &Note, time: u64) -> io::Result<()> {
        self.connection.write_all(&[
            note.pitch,
            note.velocity,
            note.duration
        ])?;
        let mut time_vec = Vec::new();
        time_vec.write_u64::<BigEndian>(time)?;
        self.connection.write_all(&time_vec)?;

        Ok(())
    }

    /// Close the connection by sending the EOF
    pub fn close(&mut self) -> io::Result<()> {
        self.connection.write_all(&[0xFF])
    }
}
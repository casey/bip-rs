use bytes::Bytes;
use std::io::Write;
use std::io;
use nom::be_u16;
use bytes::BigEndian;
use nom::IResult;
use message::bits_ext;
use message;
use byteorder::WriteBytesExt;

/// Message for notifying a peer of our DHT port.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct PortMessage {
    port: u16,
}

impl PortMessage {
    pub fn new(port: u16) -> PortMessage {
        PortMessage { port: port }
    }

    pub fn parse_bytes(_input: (), bytes: Bytes) -> IResult<(), io::Result<PortMessage>> {
        match parse_port(bytes.as_ref()) {
            IResult::Done(_, result)  => IResult::Done((), Ok(result)),
            IResult::Error(err)       => IResult::Error(err),
            IResult::Incomplete(need) => IResult::Incomplete(need)
        }
    }

    pub fn write_bytes<W>(&self, mut writer: W) -> io::Result<()>
        where W: Write
    {
        try!(message::write_length_id_pair(&mut writer, bits_ext::PORT_MESSAGE_LEN, Some(bits_ext::PORT_MESSAGE_ID)));

        writer.write_u16::<BigEndian>(self.port)
    }
}

fn parse_port(bytes: &[u8]) -> IResult<&[u8], PortMessage> {
    map!(bytes, be_u16, |port| PortMessage::new(port))
}

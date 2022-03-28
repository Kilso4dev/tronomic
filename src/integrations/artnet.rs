use crate::{
    error::DmGuiError,
    dmx::Universe,
};
use artnet_protocol as ap;
use std::net::{UdpSocket, ToSocketAddrs, IpAddr, SocketAddr};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtnetConnection {
    pub addr: String,
    #[serde(skip)]
    pub sock: Option<UdpSocket>,
}

impl std::clone::Clone for ArtnetConnection {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.clone(),
            sock: None,
        }
    }
}

impl ArtnetConnection {
    pub fn new<S: ToString>(addr: S) -> Self {
        Self {
            addr: addr.to_string(),
            sock: None,
        }
    }

    pub fn establish_connection(&mut self) -> Result<(), DmGuiError> {
        if let None = self.sock {
            self.sock = Some(UdpSocket::bind(&self.addr)?);
        }
        Ok(())
    }

    pub fn send_universe(&self, universe_id: usize, un: Universe) -> Result<(), DmGuiError> {
        let data = (0..255).map(|i| *un.get_channel(i).unwrap().get()).collect::<Vec<u8>>().into();
        let cmd = ap::ArtCommand::Output(ap::Output {
            data,
            ..ap::Output::default()
        });

        if let Some(sock) = self.sock.as_ref() {
            let buf = cmd.write_to_buffer()?;
            let sent_len = sock.send(&buf[..])?;
            if sent_len == buf.len() {
                Ok(())
            } else {
                Err(DmGuiError::networking(format!("sent length {sent_len} is not equal to byte length to be sent ({})", buf.len())))
            }
            
        } else {
            Err(DmGuiError::networking(format!("socket of configured universe {universe_id} and address {} not bound. Maybe you forgot to call \"establish_connection()\"", self.addr)))
        }
    }

    pub fn handle_incoming(&self) -> Result<ap::ArtCommand, DmGuiError> {
        let mut buf = [0; 1024];
        if let Some(sock) = self.sock.as_ref() {
            let (length, addr) = sock.recv_from(&mut buf).unwrap();
            log::info!("incoming: addr: {addr:?}");
            Ok(ap::ArtCommand::from_buffer(&buf[..length])?)
        } else {
            Err(DmGuiError::networking(format!("socket of address {} not bound. Maybe you forgot to call \"establish_connection()\"", self.addr)))
        }
    }
}

/*
let socket = UdpSocket::bind(("0.0.0.0", 6454)).unwrap();
let broadcast_addr = ("255.255.255.255", 6454).to_socket_addrs().unwrap().next().unwrap();
socket.set_broadcast(true).unwrap();
let buff = ArtCommand::Poll(Poll::default()).write_to_buffer().unwrap();
socket.send_to(&buff, &broadcast_addr).unwrap();

loop {
    let mut buffer = [0u8; 1024];
    let (length, addr) = socket.recv_from(&mut buffer).unwrap();
    let command = ArtCommand::from_buffer(&buffer[..length]).unwrap();

    println!("Received {:?}", command);
    match command {
        ArtCommand::Poll(poll) => {
            // This will most likely be our own poll request, as this is broadcast to all devices on the network
        },
        ArtCommand::PollReply(reply) => {
            // This is an ArtNet node on the network. We can send commands to it like this:
            let command = ArtCommand::Output(Output {
                data: vec![1, 2, 3, 4, 5].into(), // The data we're sending to the node
                ..Output::default()
            });
            let bytes = command.write_to_buffer().unwrap();
            socket.send_to(&bytes, &addr).unwrap();
        },
        _ => {}
    }
}*/


use ring::digest;
use crate::server::KeyFormat;

#[derive(Debug)]
pub struct Talker {
    pub pubkey : PubKey,
    pub id: String,
    pub name: String,
    state: ConnectionState,
    connection: Connection,
    password: Vec<u8>,
}

impl Talker {
    pub fn new(key: PubKey, name:&str, id:&str, password: &str) -> Talker {
        let sha = digest::digest(&digest::SHA256, password.as_bytes());
        let sha = Vec::from(sha.as_ref());
        Talker {
            pubkey: key,
            name: String::from(name),
            id: String::from(id),
            state: ConnectionState::DISCONNECT,
            connection: Connection::new(),
            password: sha,
        }
    }

    pub fn password(&self) -> &[u8] {
        &self.password[..]
    }
}

#[derive(Debug)]
pub struct PubKey {
    keyfmt: KeyFormat,
    key: Vec<u8>,
}

impl PubKey {
    pub fn from_sshkey(key_str: &str) -> Option<PubKey> {
        // TODO
        let mut data:Vec<u8> = Vec::new();
        for b in key_str.bytes() {
            data.push(b);
        }

        Some(PubKey {
            keyfmt: KeyFormat::SshKey,
            key: data,
        })
    }

    pub fn compare_key(&self, key: &PubKey) -> bool {
        self.keyfmt == key.keyfmt && self.key == key.key
    }
}

#[derive(Debug)]
enum ConnectionState {
    DISCONNECT,
    CONNECTED,
}

#[derive(Debug)]
struct Connection {
}

impl Connection {
    
    fn new() -> Connection {
        Connection {}
    }
}
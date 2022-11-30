pub struct Talker {
    pubkey : PubKey,
    id: String,
    name: String,
    state: ConnectionState,
    connection: Connection,
}

impl Talker {
    pub fn new(key: &[u8], name:&str, id:&str, ) -> Talker {
        let pkey = PubKey::from_sshkey(key);
        Talker {
            pubkey: pkey,
            name: String::from(name),
            id: String::from(id),
            state: ConnectionState::DISCONNECT,
            connection: Connection::new(),
        }
    }
}

enum PubKeyFmt {
    SshKeyFile,
}

pub struct PubKey {
    keyfmt: PubKeyFmt,
    key: Vec<u8>,
}

impl PubKey {
    fn from_sshkey(data: &[u8]) -> PubKey {

        PubKey {
            keyfmt: PubKeyFmt::SshKeyFile,
            key: Vec::new(),
        }
    }
}
enum ConnectionState {
    DISCONNECT,
    CONNECTED,
}

struct Connection {
}

impl Connection {
    
    fn new() -> Connection {
        Connection {}
    }
}
use crate::state::HashState;

mod state;

/// The transcript type that keeps track of the internal hash state.
#[derive(Clone)]
pub struct Transcript {
    state: HashState,
}

impl Transcript {
    /// Create a new transcript.
    pub fn new(label: &'static [u8]) -> Transcript {
        let mut transcript = Transcript {
            state: HashState::new(label),
        };
        transcript.append_message(b"dom-sep", label);
        transcript
    }

    /// Append a message to the transcript.
    pub fn append_message(&mut self, label: &'static [u8], message: &[u8]) {
        let data_len = encode_usize_as_u32(message.len());
        self.state.absorb(label);
        self.state.absorb(&data_len);
        self.state.absorb(message);
    }

    /// Append a `u64` number into the transcript.
    pub fn append_u64(&mut self, label: &'static [u8], x: u64) {
        self.append_message(label, &encode_u64(x));
    }

    /// Squeeze bytes out of the transcript into a destination buffer.
    pub fn challenge_bytes(&mut self, label: &'static [u8], dest: &mut [u8]) {
        // enforce that the destination buffer must be a multiple of 32 for now
        assert!(dest.len() % 32 == 0);

        let data_len = encode_usize_as_u32(dest.len());
        self.state.absorb(label);
        self.state.absorb(&data_len);

        for chunk in dest.chunks_mut(32) {
            let hash = self.state.squeeze();
            chunk.copy_from_slice(hash.as_ref());
        }
    }
}

fn encode_u64(x: u64) -> [u8; 8] {
    use byteorder::{ByteOrder, LittleEndian};

    let mut buf = [0; 8];
    LittleEndian::write_u64(&mut buf, x);
    buf
}

fn encode_usize_as_u32(x: usize) -> [u8; 4] {
    use byteorder::{ByteOrder, LittleEndian};

    assert!(x <= (u32::max_value() as usize));

    let mut buf = [0; 4];
    LittleEndian::write_u32(&mut buf, x as u32);
    buf
}

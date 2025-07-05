use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
        CompactSize { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)

        let value = self.value;
        match value {
            0x00..=0xFC => vec![value as u8],
            0xFD..=0xFFFF => {
                let mut bytes = vec![0xFD];
                bytes.extend_from_slice(&(value as u16).to_le_bytes());
                bytes
            }
            0x10000..=0xFFFFFFFF => {
                let mut bytes = vec![0xFE];
                bytes.extend_from_slice(&(value as u32).to_le_bytes());
                bytes
            }
            _ => {
                let mut bytes = vec![0xFF];
                bytes.extend_from_slice(&(value).to_le_bytes());
                bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.

        if bytes.is_empty() {
            return Err(BitcoinError::InsufficientBytes);
        }
        match bytes[0] {
            0x00..=0xFC => {
                let value = bytes[0] as u64;
                Ok((CompactSize::new(value), 1))
            }
            0xFD => {
                if bytes.len() < 3 {
                    return Err(BitcoinError::InsufficientBytes);
                }

                let value = u16::from_le_bytes([bytes[1], bytes[2]]) as u64;
                if value < 0xFD {
                    return Err(BitcoinError::InvalidFormat);
                }
                Ok((CompactSize::new(value), 3))
            }
            0xFE => {
                if bytes.len() < 5 {
                    return Err(BitcoinError::InsufficientBytes);
                }
                let value = u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]) as u64;
                if value <= 0xFFFF {
                    return Err(BitcoinError::InvalidFormat);
                }
                Ok((CompactSize::new(value), 5))
            }
            0xFF => {
                if bytes.len() < 9 {
                    return Err(BitcoinError::InsufficientBytes);
                }
                let value = u64::from_le_bytes([
                    bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
                ]);
                if value <= 0xFFFFFFFF {
                    return Err(BitcoinError::InvalidFormat);
                }
                Ok((CompactSize::new(value), 9))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
        let hex_str = hex::encode(self.0);
        serializer.serialize_str(&hex_str)
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32
        let hex_str = match String::deserialize(deserializer) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let bytes = match hex::decode(&hex_str) {
            Ok(bytes) => bytes,
            Err(_) => return Err(D::Error::custom("Invalid hex string")),
        };

        if bytes.len() != 32 {
            return Err(D::Error::custom("Hex string must be 32 bytes"));
        }

        let array: [u8; 32] = bytes.try_into().unwrap();

        Ok(Txid(array))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
        OutPoint {
            txid: Txid(txid),
            vout,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.txid.0);
        bytes.extend_from_slice(&self.vout.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let txid = Txid(bytes[0..32].try_into().unwrap());
        let vout = u32::from_le_bytes(bytes[32..36].try_into().unwrap());

        Ok((OutPoint { txid, vout }, 36))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
        Script { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let mut buffer = Vec::new();

        let compact_size_len = CompactSize::new(self.bytes.len() as u64).to_bytes();
        buffer.extend(compact_size_len);

        buffer.extend(&self.bytes);

        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        // Read the CompactSize prefix
        let parse_output = CompactSize::from_bytes(bytes);
        let (compact_size_len, var_int_len) = match parse_output {
            Ok((compact_size_value, bytes_count)) => (compact_size_value, bytes_count),
            Err(_) => return Err(BitcoinError::InsufficientBytes),
        };

        let script_len = compact_size_len.value as usize;

        let total_len = var_int_len + script_len;
        if bytes.len() < total_len {
            return Err(BitcoinError::InsufficientBytes);
        }

        let script_bytes = bytes[var_int_len..total_len].to_vec();

        Ok((
            Script {
                bytes: script_bytes,
            },
            total_len,
        ))
    }
}

impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
        &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        TransactionInput {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.previous_output.to_bytes());
        bytes.extend_from_slice(&self.script_sig.to_bytes());
        bytes.extend_from_slice(&self.sequence.to_le_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)
        if bytes.len() < 41 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let txid = Txid(bytes[0..32].try_into().unwrap());
        let vout = u32::from_le_bytes(bytes[32..36].try_into().unwrap());
        let previous_output = OutPoint { txid, vout };

        let (script_sig, script_bytes) = match Script::from_bytes(&bytes[36..]) {
            Ok((script, bytes_used)) => (script, bytes_used),
            Err(_) => return Err(BitcoinError::InsufficientBytes),
        };

        let sequence_start = 36 + script_bytes;

        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let sequence_bytes = &bytes[sequence_start..sequence_start + 4];
        let sequence_array = match sequence_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err(BitcoinError::InvalidFormat),
        };

        let sequence = u32::from_le_bytes(sequence_array);

        let total_bytes = sequence_start + 4;

        Ok((
            TransactionInput {
                previous_output,
                script_sig,
                sequence,
            },
            total_bytes,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
        Self {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.version.to_le_bytes());

        let input_count = CompactSize::new(self.inputs.len() as u64).to_bytes();
        bytes.extend(input_count);
        for input in &self.inputs {
            bytes.extend(input.to_bytes());
        }

        bytes.extend_from_slice(&self.lock_time.to_le_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for lock_time
        todo!()
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
        todo!()
    }
}

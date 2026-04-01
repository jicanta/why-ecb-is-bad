use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};

fn pad_to_block(data: &[u8]) -> Vec<u8> {
    let mut out = data.to_vec();
    let rem = out.len() % 16;
    if rem != 0 {
        out.resize(out.len() + (16 - rem), 0);
    }
    out
}

pub fn encrypt_ecb(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut out = pad_to_block(data);
    for chunk in out.chunks_mut(16) {
        cipher.encrypt_block(GenericArray::from_mut_slice(chunk));
    }
    out
}

pub fn encrypt_cbc(data: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut out = pad_to_block(data);
    let mut prev = *iv;
    for chunk in out.chunks_mut(16) {
        for (b, p) in chunk.iter_mut().zip(prev.iter()) {
            *b ^= p;
        }
        cipher.encrypt_block(GenericArray::from_mut_slice(chunk));
        prev.copy_from_slice(chunk);
    }
    out
}

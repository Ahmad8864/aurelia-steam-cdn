use aes::{
    Aes256, Aes256Dec,
    cipher::{
        Block, BlockCipherDecrypt, BlockModeDecrypt, KeyInit, KeyIvInit,
        block_padding::{self, Pkcs7},
    },
};

pub const IV_LENGTH: usize = 16;

pub fn decrypt_cbc_with_iv_extraction(
    data: &mut [u8],
    key: [u8; 32],
) -> Result<Vec<u8>, block_padding::Error> {
    let mut iv = [0u8; IV_LENGTH];
    iv.copy_from_slice(&data[..IV_LENGTH]);
    let mut iv = Block::<Aes256Dec>::from(iv);
    Aes256Dec::new(&key.into()).decrypt_block(&mut iv);

    Ok(cbc::Decryptor::<Aes256>::new(&key.into(), &iv)
        .decrypt_padded::<Pkcs7>(&mut data[IV_LENGTH..])?
        .to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Key bytes 0..32; vector made with openssl.
    const KEY: [u8; 32] = {
        let mut k = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            k[i] = i as u8;
            i += 1;
        }
        k
    };
    const VECTOR: &str = "dc9f01fd76b87c0d0f4be8850b0bece0a1dc09e92611df1929f89e2576e9c062a1be3bc12fc786665ed6cac9123c832f";

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn decrypts_steam_style_payload() {
        let mut data = hex(VECTOR);
        let plain = decrypt_cbc_with_iv_extraction(&mut data, KEY).unwrap();
        assert_eq!(plain, b"bin/game_x64.exe");
    }

    #[test]
    fn wrong_key_fails_padding() {
        let mut data = hex(VECTOR);
        let mut key = KEY;
        key[0] ^= 1;
        assert!(decrypt_cbc_with_iv_extraction(&mut data, key).is_err());
    }
}

use aes::{
    Aes256, Aes256Dec,
    cipher::{
        BlockDecrypt, BlockDecryptMut, KeyInit, KeyIvInit,
        block_padding::{Pkcs7, UnpadError},
        generic_array::GenericArray,
    },
};

pub const IV_LENGTH: usize = 16;

pub fn decrypt_cbc_with_iv_extraction(
    data: &mut [u8],
    key: [u8; 32],
) -> Result<Vec<u8>, UnpadError> {
    let mut iv = [0u8; IV_LENGTH];
    iv.copy_from_slice(&data[..IV_LENGTH]);
    Aes256Dec::new(GenericArray::from_slice(&key))
        .decrypt_block(GenericArray::from_mut_slice(&mut iv[..]));

    Ok(cbc::Decryptor::<Aes256>::new(
        GenericArray::from_slice(&key),
        GenericArray::from_slice(&iv),
    )
    .decrypt_padded_mut::<Pkcs7>(&mut data[IV_LENGTH..])?
    .to_vec())
}

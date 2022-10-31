use anyhow::{Result};

use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::{blockmodes::PkcsPadding, buffer::{RefReadBuffer, RefWriteBuffer}};

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
  tracing::debug!("Key size: {}", key.len());

  let mut encryptor = crypto::aes::ecb_encryptor(crypto::aes::KeySize::KeySize256, key, PkcsPadding);

  let mut final_result = vec![];
  let mut input = RefReadBuffer::new(plaintext);
  let mut buffer = [0; 4096];
  let mut output = RefWriteBuffer::new(&mut buffer);

  loop {
    let result = encryptor.encrypt(&mut input, &mut output, true)
      .map_err(|error| anyhow::anyhow!("AES encryption failed: {:?}", error))?;

    final_result.extend(output.take_read_buffer().take_remaining().iter().map(|&i| i));

    match result {
      BufferResult::BufferUnderflow => break,
      _ => {}
    }
  }

  Ok(final_result)
}

pub fn encrypt_text(plaintext : &str, key: &[u8]) -> Result<Vec<u8>> {
  encrypt(plaintext.as_bytes(), key)
}

pub fn decrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
  let mut decryptor = crypto::aes::ecb_decryptor(crypto::aes::KeySize::KeySize256, key, PkcsPadding);

  let mut final_result = vec![];
  let mut input = RefReadBuffer::new(plaintext);
  let mut buffer = [0; 4096];
  let mut output = RefWriteBuffer::new(&mut buffer);

  loop {
    let result = decryptor.decrypt(&mut input, &mut output, true)
      .map_err(|error| anyhow::anyhow!("AES encryption failed: {:?}", error))?;

    final_result.extend(output.take_read_buffer().take_remaining().iter().map(|&i| i));

    match result {
      BufferResult::BufferUnderflow => break,
      _ => {}
    }
  }

  Ok(final_result)
}

pub fn decrypt_text(plaintext: &[u8], key: &[u8]) -> Result<String> {
  let bytes = decrypt(plaintext, key)?;
  Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod test {
  use anyhow::Result;
  use super::*;

  #[test]
  fn it_encrypts_data() -> Result<()> {
    let result = encrypt("hello".as_bytes(), "bdadd4ef579ebe5a1ff92e2785686b2c".as_bytes())?;
    assert_eq!(vec![30, 230, 75, 6, 223, 227, 195, 53, 103, 147, 179, 220, 158, 174, 82, 210], result);
    Ok(())
  }

  #[test]
  fn it_encrypts_text() -> Result<()> {
    let result = encrypt_text("hello", "bdadd4ef579ebe5a1ff92e2785686b2c".as_bytes())?;
    assert_eq!(vec![30, 230, 75, 6, 223, 227, 195, 53, 103, 147, 179, 220, 158, 174, 82, 210], result);
    Ok(())
  }

  #[test]
  fn it_decrypt_data() -> Result<()> {
    let result = decrypt(&vec![30, 230, 75, 6, 223, 227, 195, 53, 103, 147, 179, 220, 158, 174, 82, 210], "bdadd4ef579ebe5a1ff92e2785686b2c".as_bytes())?;
    assert_eq!("hello".as_bytes(), result);
    Ok(())
  }

  #[test]
  fn it_decrypt_text() -> Result<()> {
    let result = decrypt_text(&vec![30, 230, 75, 6, 223, 227, 195, 53, 103, 147, 179, 220, 158, 174, 82, 210], "bdadd4ef579ebe5a1ff92e2785686b2c".as_bytes())?;
    assert_eq!("hello", result);
    Ok(())
  }
}

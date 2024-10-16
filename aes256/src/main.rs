use anyhow::{anyhow, Result};
use base64::Engine;
use crypto::aes::{cbc_decryptor,cbc_encryptor};
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};

fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    //检查密钥和 IV 的长度
    if key.len() != 32 {
        return Err(anyhow!("Invalid key length"));
    }
    if iv.len() != 16 {
        return Err(anyhow!("Invalid IV length"));
    }
    let mut decryptor = cbc_decryptor(KeySize256, key, iv, PkcsPadding);
    let mut buffer = vec![0; ciphertext.len()];
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(ciphertext);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
    let decrypted_len = write_buffer.position() as usize;
    buffer.truncate(decrypted_len);

    Ok(buffer)
}

fn encrypt(message: &str, key: &[u8], iv: &[u8]) -> Result<String> {
    // 检查密钥和 IV 的长度
    if key.len() != 32 {
        return Err(anyhow!("Invalid key length"));
    }
    if iv.len() != 16 {
        return Err(anyhow!("Invalid IV length"));
    }

    let mut encryptor = cbc_encryptor(KeySize256, key, iv, PkcsPadding);

    // 准备输入和输出的缓冲区
    let mut input = RefReadBuffer::new(message.as_bytes());
    let mut output = vec![0; message.len() + 16]; // 预留足够的空间来存放加密后的数据

    let mut output_buffer = RefWriteBuffer::new(&mut output);

    // 加密消息
    let result = encryptor.encrypt(&mut input, &mut output_buffer, true);
    match result {
        Ok(BufferResult::BufferUnderflow) => {
            // 如果加密成功，获取实际使用的字节数, 仅保留有效数据的部分
            let final_length = output_buffer.position();
            output.truncate(final_length);
        }
        Ok(BufferResult::BufferOverflow) => {
            return Err(anyhow!("Buffer overflow occurred during encryption"));
        }
        Err(e) => {
            return Err(anyhow!("Encryption failed: {:?}", e));
        }
    }

    // data-> Base64
    let encrypted_base64 = base64::encode(&output);
    Ok(encrypted_base64)
}

fn main() -> Result<()> {
    let key = "Xbri5bnNHeZoOWYbkVk6NUfkeNxhsIq8".as_bytes().to_vec();
    let iv = "kFogIuFPXX9FXJKq".as_bytes().to_vec();
    //let data = base64::engine::general_purpose::STANDARD.decode("rgmmrmL87NuajzPJY/AJRA==".to_string())?;

    let message = "root";
    let encrypted_message = encrypt(message, &key, &iv)?;
    println!("Encrypted message (Base64): {}", encrypted_message);
    let data = base64::engine::general_purpose::STANDARD.decode("tiYS7CuM1X6QM45O+pbasA==")?;
    let decrypted_data = decrypt(&data, &key, &iv)?;

    match String::from_utf8(decrypted_data) {
        Ok(decrypted_message) => {
            println!("Decrypted message: {}", decrypted_message);
        }
        Err(e) => {
            println!("Failed to decode UTF-8: {}", e);
        }
    }

    Ok(())
}
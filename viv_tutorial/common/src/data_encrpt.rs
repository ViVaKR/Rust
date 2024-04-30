use data_encoding::HEXUPPER;
use ring::digest::{self, Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::hmac::{self};
use ring::pbkdf2;
use ring::rand::{self, SecureRandom};
use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::num::NonZeroU32;

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer).expect("Error");
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}

pub fn encrpt_run() -> Result<(), Error> {
    let path = "file.txt";

    let mut output = File::create(path).expect("Error");
    let w = write!(output, "We will generate a digest of this text");
    let rs = match w {
        Ok(val) => val,
        Err(e) => println!("\u{26EC} {}", e),
    };

    println!("\u{26EC} {:#?}", rs);
    let input = File::open(path).expect("Error");
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!(
        "\u{26EC} SHA-256 digest is {}",
        HEXUPPER.encode(digest.as_ref())
    );

    Ok(())
}

pub fn signature_hmac_run() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];

    let rng = rand::SystemRandom::new();
    _ = rng.fill(&mut key_value);
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    println!("\u{26EC} message: {}\nsignature: {:?}", message, signature);
    Ok(())
}

pub fn cipher_run() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    _ = rng.fill(&mut salt);

    let password = "Guess Me If You Can!";

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    println!("\u{26EC} Salt: {}", HEXUPPER.encode(&salt));
    println!("\u{26EC} PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );

    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}

use argon2::{self, Argon2};
use chacha20poly1305::{
	aead::{Aead, AeadCore, KeyInit, OsRng},
	XChaCha20Poly1305, XNonce,
};
use hex;
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 4 {
		eprintln!(
			"Usage: {} <E(encrypt)|D(decrypt)> <file> <password> <output>",
			args[0]
		);
		std::process::exit(1);
	}

	let mode = &args[1];
	let file = &args[2];
	let passwd = &args[3];
	let output = if args.len() > 4 { Some(&args[4]) } else { None };

	match mode.as_str() {
		"E" => encrypt(file, passwd, output.map(|x| x.as_str())),
		"D" => decrypt(file, passwd, output.map(|x| x.as_str())),
		_ => {
			eprintln!("Invalid mode. Use 'E(encrypt)' or 'D(decrypt)'.");
			std::process::exit(1);
		}
	}
}

fn encrypt(file: &str, passwd: &str, output: Option<&str>) {
	let content = fs::read(file).expect("Failed to read file");

	let salt = b"rcipher-salt-thing";
	let mut key = [0u8; 32];
	Argon2::default()
		.hash_password_into(passwd.as_bytes(), salt, &mut key)
		.expect("Failed to derive key");

	let cipher = XChaCha20Poly1305::new_from_slice(&key).expect("Failed to create cipher");
	let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

	let cipherText = cipher
		.encrypt(&nonce, content.as_ref())
		.expect("Encryption failed");

	let encrypted_data = format!(
		"{}:{}:{}",
		hex::encode(key),
		hex::encode(nonce),
		hex::encode(cipherText)
	);

	let default = format!("{}.rcp", file);
	let output = output.unwrap_or(&default);
	let output = if output.ends_with(".rcp") {
		output.to_string()
	} else {
		format!("{}.rcp", output)
	};

	fs::write(output, encrypted_data).expect("Failed to write encrypted file");
	println!("File encrypted successfully.");
}

fn decrypt(file: &str, passwd: &str, output: Option<&str>) {
	let content = fs::read_to_string(file).expect("Failed to read file");
	let parts: Vec<&str> = content.split(':').collect();
	if parts.len() != 3 {
		eprintln!("Invalid encrypted file format");
		std::process::exit(1);
	}

	let key = hex::decode(parts[0]).expect("Failed to decode key");
	let nonce = hex::decode(parts[1]).expect("Failed to decode nonce");
	let cipherText = hex::decode(parts[2]).expect("Failed to decode ciphertext");

	let salt = b"rcipher-salt-thing";
	let mut derived_key = [0u8; 32];
	Argon2::default()
		.hash_password_into(passwd.as_bytes(), salt, &mut derived_key)
		.expect("Failed to derive key");

	if key != derived_key {
		eprintln!("Incorrect password");
		std::process::exit(1);
	}

	let cipher = XChaCha20Poly1305::new_from_slice(&key).expect("Failed to create cipher");
	let nonce = XNonce::from_slice(&nonce);

	let plain_text = cipher
		.decrypt(nonce, cipherText.as_ref())
		.expect("Decryption failed");

	let default = file.replace(".rcp", "");
	let output = output.unwrap_or(&default);
	fs::write(output, plain_text).expect("Failed to write decrypted file");
	println!("File decrypted successfully.");
}

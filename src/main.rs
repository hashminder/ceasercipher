use rand::Rng;

fn random_bytes(len: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut v = vec![0u8; len];
    rng.fill_bytes(&mut v);
    v
}


fn encrypt (plaintext : String,key : u8) -> String {
    let shift_key: u8 = key;
    let message = plaintext.as_bytes();
    let mut cipher : Vec<u8> = Vec::new();
    for i in message {
        cipher.push((i+ shift_key) % 128);
    };
     return String::from_utf8(cipher).expect("failed to convert");
}


fn main() {
    let mut i = 1;
    loop {
        let plaintext = "random_bytes(16)";
        println!("{}",encrypt(plaintext.to_string(),i));
        i += 1;
        println!("{}",i);
    }
}

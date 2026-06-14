fn encrypt (plaintext : String,key : u8) -> String {
    let shift_key: u8 = key % 126;
    let message = plaintext.as_bytes();
    let mut cipher : Vec<u8> = Vec::new();
    for i in message {
        cipher.push((i+ shift_key) % 128);
    };
    if cipher == plaintext.as_bytes() {
        println!("change your key... it wont loop the cipher forward");
        return "0".to_string()
    }
     return String::from_utf8(cipher).expect("failed to convert");
}

fn mod_backward (n : &u8,subtract : u8,mod_field : u8) -> u8 {
    let go_forward = mod_field - (subtract % mod_field );
     (n + go_forward) % mod_field
}

fn decrypt (cipher : String,key : u8) -> String {
    let cipher = cipher.as_bytes();
    let mut plain_text = Vec::new();
    for i in cipher {
        plain_text.push(i - key);
    }
    return String::from_utf8(plain_text).expect("failed to convert dec")
}

fn brute_force(cipher : String) -> Vec<String> {
    let cipher = cipher.as_bytes();
    let mut brute_array = Vec::new();
    for i in 0..=255 {
        let mut assemble = Vec::new();
        for letter in cipher {
            assemble.push(mod_backward(letter,i,127));
        }
        brute_array.push(String::from_utf8(assemble).expect("failed to convert the data"));
    }
    return brute_array
}

fn main() {
    let key = 3;
    let plaintext = "some random text".to_string();
    let c = encrypt(plaintext,key);
    let _b = brute_force(c.clone());
    println!("{:#?}",brute_force(c.clone()));
    println!("{}",decrypt(c,key));
}

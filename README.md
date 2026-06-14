Hello !
( READ THE README.txt AS CODE VIEW FOR BETTER LOOK)

use encrypt function to encrypt your plaintext :
  function overview : fn encrypt (plaintext : String,key : u8) -> String
  plaintext = your string
  key = shift number
  EXAMPLE USAGE :
    let c = encrypt("some data as STRING".to_string(),5);

use decrypt function to decrypt your cipher :
  function overview : fn decrypt (cipher : String,key : u8) -> String 
  cipher : your cipher
  key : you shift number used to encrypt cipher
  EXAMPLE USAGE :
    println!("{}",decrypt("Lw#zrunv#$".to_string(),key));

use brute_force to guess all possible states of the plain text :
  function overview : fn brute_force(cipher : String) -> Vec<String> 
  cipher = your cipher

  this function returns a vector of possible plaintexts, one of them is readable, that's your plain text
  let _b = brute_force(cipher);

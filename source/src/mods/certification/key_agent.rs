use secp256k1::{Secp256k1, SecretKey, };
use std::{str::FromStr, path::Path};
use rand::{rngs::OsRng};
use std::fs::{File,OpenOptions};
use crate::mods::util::system;
use std::io::{prelude::*,BufReader,Write};
pub static mut SECRET:Vec<SecretKey>=Vec::new();

/*  
    implements later from https://github.com/rust-bitcoin/rust-secp256k1/blob/master/examples/sign_verify_recovery.rs
    and https://docs.rs/secp256k1/latest/secp256k1/ 
*/
//create file or read file
 pub fn init(){
    let mut is_exst=false;
    if Path::new("secret/secret.txt").exists() {
        is_exst=true;
    }
    let f=OpenOptions::new().create(true).read(true).write(true).open("secret/secret.txt").unwrap();
    if !is_exst {
        create_new_key();
    }
    unsafe{
        read_key_from_file(f);
        assert!(SECRET.len()>0);
    }
    
}
fn create_new_key(){
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    append_key_to_file(secret_key);
}
unsafe fn read_key_from_file(file:File){
    let mut svec:Vec<SecretKey>=  Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let key:SecretKey=SecretKey::from_str(&line).unwrap();
        svec.push(key);
    }
    for _ in 0..svec.len(){
        SECRET.push(svec.pop().unwrap());
    }
}
fn append_key_to_file(key:SecretKey){
    let mut f=OpenOptions::new().read(true).write(true).create(true).open(Path::new("secret/secret.txt")).unwrap();
    let secret_str=format!("{}\n",key.display_secret());
    f.write(secret_str.as_bytes()).unwrap();
    f.flush().unwrap();
}
pub fn get_key(index:usize)->Option<&'static SecretKey>{
    unsafe{
        SECRET.get(index)
    }
}
pub fn get_key_length()->usize{
    unsafe{
        SECRET.len()
    }
}
#[test]
fn key_agent_init(){
    init();
    for i in 0..(get_key_length()){
        println!("{}",get_key(i).unwrap().display_secret());
    }
}
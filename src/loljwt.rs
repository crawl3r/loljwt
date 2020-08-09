/*
    Used EroDir as a ref when building this out, thank you Pinky :) appreciate you!
    https://github.com/PinkP4nther/EroDir

    p.s:
    Anyone who uses Rust, this is my first project in the language. If I butchered it, I apologise but please help me be better <3 
*/

extern crate loljwtlib;
extern crate serde;
extern crate jsonwebtoken;
extern crate clap;

use std::time::{Instant};
use clap::{App, Arg};
use loljwtlib::{TargetJwtInfo,ThreadBuildHandle};
use std::thread;
use std::fs::{File};
use std::io::{stdout,Write,BufRead,BufReader};
use std::sync::{Arc, Mutex, mpsc};
use std::process;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use jsonwebtoken::crypto::{verify};

const VERSION: &str = "0.1";

// main function of the program
fn main() {
    let args = App::new("-=[- lolJWT")
        .version(format!("{} ---------------------------------------------]=",VERSION).as_str())
        .author("=[-- @monobehaviour -------------------------------------]=")
        .about("=[-- A HS256 JWT cracker --------------------------------]=-")

        .arg(Arg::with_name("jwtToken")
            .short("j")
            .long("jwt")
            .value_name("")
            .help("JWT to crack")
            .required(true)
            .takes_value(true))

        .arg(Arg::with_name("wordlist")
            .short("w")
            .long("wordlist")
            .value_name("wordlist.lst")
            .help("Wordlist of possible secrets")
            .required(true)
            .takes_value(true))

        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .value_name("10")
            .help("Amount of threads to use (Default: 10)")
            .takes_value(true))

        .get_matches();

    let wfile = match args.value_of("wordlist") {
        Some(entry) => String::from(entry),
        _ => String::from("")
    };

    let threads = match args.value_of("threads") {
        Some(t) => match String::from(t).parse::<u32>() {
            Ok(i) => {
                if i >= 1 {
                        i
                    } else {
                        println!("[!] --threads must have more than 0 threads!");
                        return;
                    }},
            Err(_) => {
                println!("[!] --threads must be a number!"); return;
            }
        },
        None => 10
    };

    let mut loljwt_obj = TargetJwtInfo::new();
    let test_jwt = match args.value_of("jwtToken") {
        Some(j) => String::from(j),
        _ => String::from("")
    };
    
    loljwt_obj.set_jwt((&test_jwt).to_string()); // <- wtf is that
    loljwt_obj.set_thread_count(threads);

    println!("--------------------------------------------------");
    println!("--=[ lolJWT v{} ]=--------------------------------",VERSION);
    println!("--=[ @monobehaviour ]=----------------------------");
    println!("--------------------------------------------------");

    println!("[+] Entry List: \t[{}]",wfile);
    println!("[+] Threads: \t\t[{}]",threads);
    println!("--------------------------------------------------");

    println!("[*] JWT => {}\n", test_jwt);
    println!("[*] Reading wordlist...");
    let wordlistlines: Vec<String> = read_lines(&read_file(&wfile));
    println!("[*] Loaded {} words", wordlistlines.len());
    loljwt_obj.set_wordlist_lines(&wordlistlines);
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        stat_verb(rx);
        process::exit(1);
    });

    thread_gen(threads, &Arc::new(Mutex::new(loljwt_obj)), tx);
}

fn stat_verb(rx: mpsc::Receiver<u16>) {

    let mut p_flag: u32 = 0;
    let syms = vec!["-","\\","|","/"];
    let mut i: usize = 3;

    loop {
        match rx.recv() {
            Ok(_) => {
            if p_flag == 100 {
                print!(" [{}]\r",syms[i]);
                match stdout().flush() {
                    Ok(_) => {},
                    Err(_) => {return;}
                }
                if i == 3 {i = 0;} else {
                    i = i + 1;}
                p_flag = 1;
            } else {
                p_flag = p_flag + 1;
            }},
            Err(_) => {return;}
        }
    }
}// End of stat_verb

fn read_file(file_name: &String) -> File {
    match File::open(file_name) {
        Ok(f) => f,
        Err(_) => {
            println!("[!] Could not open file: {}",file_name);
            process::exit(1);
        }
    }
}// End of read_file

fn read_lines(f: &File) -> Vec<String> {
    let buf = BufReader::new(f);

    // assumes everything is UTF-8
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()

}// End of read_lines

fn thread_gen(thread_count: u32, loljwt_obj: &Arc<Mutex<TargetJwtInfo>>, tx: mpsc::Sender<u16>) {
    let mut build_handles: Vec<ThreadBuildHandle> = Vec::new();
    let mut t_handles: Vec<thread::JoinHandle<()>> = Vec::new();

    // Initialize threads
    for _ in 0..thread_count {

        let mut bh = ThreadBuildHandle::new();

        bh.robj = loljwt_obj.clone();

        build_handles.push(bh);
    }

    println!("[*] Threads Built: {}", build_handles.len());
    println!("--------------------------------------------------");

    //Spawn threads
    for th in build_handles {
        let tx = mpsc::Sender::clone(&tx);

        t_handles.push(thread::spawn(move || {
            // do the actual task here
            decode_engine(&th.robj, tx);
        }));
    }

    for th in t_handles {
        th.join().unwrap();
    }

    // ??
    let h = loljwt_obj.lock().unwrap();

    println!("[+] Finished!");
}// End of thread_gen

fn decode_engine(robj: &Arc<Mutex<TargetJwtInfo>>, tx: mpsc::Sender<u16>) {
    let mut lines: Vec<String> = Vec::new();
    let tmpl = robj.lock().unwrap();
    drop(tmpl);
    
    loop {
        // Get Mutex handle
        let mut entry = robj.lock().unwrap();

        // Check that there are still entries left
        if entry.wordlist_lines.len() == 0 {
            drop(entry);
            break;
        }

        // Get next entry 
        let e = match entry.wordlist_lines.pop() {
            Some(e) => {
                if e == "" {continue;} else {e}
                },
            None => continue
        };

        let mut msg = entry.msg.clone();
        let mut sig = entry.sig.clone();
        let secret = e.as_str();
        
        let found = try_decode_jwt(&msg, &sig, secret.to_string());
        tx.send(1).unwrap();

        if found {
            // this is awful because threaded, but yolo. Todo: learn how to quit safely?
            process::exit(1);
        }
        drop(entry);
    }
}// End of decode_engine

fn try_decode_jwt(msg: &String, sig: &String, secret: String) -> bool {
    /*
        pub fn verify(
            signature: &str, 
            message: &str, 
            key: &DecodingKey, 
            algorithm: Algorithm
        ) -> Result<bool>

        signature is the signature part of a jwt (text after the second '.')
        message is base64(header) + "." + base64(claims)
    */

    match verify(sig, msg, &DecodingKey::from_secret(secret.as_ref()), Algorithm::HS256) {
        Ok(c) => {
            if c {
                println!("[*]  Secret found: {}", secret);
                return true;
            } else {
                return false;
            }
        },
        Err(err) => {
            println!("Failed");
            panic!("Derp: {}", err);
        }
    };
}// End of try_decode_jwt
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct TargetJwtInfo {
    pub jwt: String,
    pub msg: String,
    pub sig: String,
    pub thread_count: u32,
    pub wordlist_lines: Vec<String>,
}

impl TargetJwtInfo {
    pub fn new() -> TargetJwtInfo {
        TargetJwtInfo {
            jwt: String::new(),
            msg: String::new(),
            sig: String::new(),
            thread_count: 0,
            wordlist_lines: Vec::new(), 
        }
    }

    pub fn set_jwt(&mut self, jwt: String) {
        self.jwt = jwt.clone();

        // signature is the signature part of a jwt (text after the second '.')
        // message is base64(header) + "." + base64(claims)

        // NOTE: This is horrible. It literally took me ages to figure out how to try chip this up at the '2nd' '.' char.... fml 
        let mut splitJwt = jwt.split('.');
        let msg1 = splitJwt.next().unwrap();
        let msg2 = splitJwt.next().unwrap();
        
        let mut msg = msg1.to_string();
        msg.push_str(".");
        msg.push_str(&msg2.to_string());

        let sig = splitJwt.next().unwrap();

        self.msg = msg.to_string();
        self.sig = sig.to_string();
    }

    pub fn set_thread_count(&mut self, tc: u32) {
        self.thread_count = tc;
    }

    pub fn set_wordlist_lines(&mut self, wl: &Vec<String>) {
        self.wordlist_lines = wl.clone();
    }

    pub fn zero_out(&mut self) {
        self.wordlist_lines = Vec::new();
    }
}

pub struct ThreadBuildHandle {
    pub robj: Arc<Mutex<TargetJwtInfo>>,
}

impl ThreadBuildHandle {
    pub fn new() -> ThreadBuildHandle {
        ThreadBuildHandle {
            robj: Arc::new(Mutex::new(TargetJwtInfo::new())),
        }
    }
}
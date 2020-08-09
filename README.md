# LOLJWT?!

A jwt cracker written in Rust. Apologies for any bad code, this is my first attempt at Rust - but I'm very much enjoying it :)

Build in release, works so much faster (for obvious reasons):  
```
cargo build --release
```

Running:  
```
./target/release/loljwt --jwt "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE1OTcwMDMyMDAsImV4cCI6MTYyODUzOTIwMCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoianJvY2tldEBleGFtcGxlLmNvbSIsIkdpdmVuTmFtZSI6IkpvaG5ueSIsIlN1cm5hbWUiOiJSb2NrZXQiLCJFbWFpbCI6Impyb2NrZXRAZXhhbXBsZS5jb20iLCJSb2xlIjpbIk1hbmFnZXIiLCJQcm9qZWN0IEFkbWluaXN0cmF0b3IiXX0.-Iux6sitT_bVWQr8jm-5OEpqmnFd6_Ndgz--nudg4X8" --wordlist ~/Downloads/rockyou.txt -t 20
```

Note: When parsing the wordlist, I have only accounted for UTF-8 stuff so skip any non-friendly first:  
```
iconv -f utf-8 -t utf-8 -c old.txt > new.txt
```

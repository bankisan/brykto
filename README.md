# brykto

```
brykto (broken crypto)

     8 8 8 8                     ,ooo.
     8a8 8a8                    oP   ?b
    d888a888zzzzzzzzzzzzzzzzzzzz8     8b
     `""^""'                    ?o___oP'
```

> 🚧 This crate is for educational purposes only. It's a homegrown rewrite of several cryptographic libraries and protocols that should **NOT** be used in production. 🚧

## Why?
As the name implies, this is a broken cryptography crate. Its aim is to tear apart cryptography libraries and peek inside. Exposing the [blackbox](https://en.wikipedia.org/wiki/Black_box). Over time, the aim is to implement the most widely used algorithms and some esoteric ones as well.

Integration tests and benchmarks will have answers to cryptography problem sets in the wild. Contained within are answers to [cryptopals](https://cryptopals.com/), [uncloak course homework](https://uncloak.org/courses/rust+cryptography+engineering/course-2022-11-18+Session+1+Notes), and others. While the aim is to rewrite most libraries, several answers will use implementations from the `RustCrypto` crate until they have been rewritten.

### Planned TODOs

##### Block ciphers
- [ ] AES
	- [ ] ECB
	- [ ] CBC
	- [ ] CRT

##### Hashers
- [X] SHA1
- [ ] SHA256, SHA512
- [ ] SHA3
- [ ] keccak256
- [X] MD4
- [ ] MD5
- [ ] Pedersen hash

##### PRNGs
- [ ] MT19937 Mersenne Twister

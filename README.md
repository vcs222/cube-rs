# cube-rs
[![license: MIT](https://img.shields.io/badge/license-MIT-green)](https://github.com/vcs222/cube-rs/blob/master/LICENSE)

Rubik's Cube Scramble Generator

Build a release version:
```
cargo build --release
```

Install `cube-rs` globally:
```
sudo cp target/release/cube-rs /usr/local/bin
```

Generate a scramble sequence:
```
cube-rs
```
output:
```
U2 R' U' B2 D2 U' D' L' B2 D' R B F' R2 F' L2 R' B F2 B' L B' F' B F2 
```
Specify the length of the sequence:
```
cube-rs [length_of_sequence]
cube-rs 42
```
output:
```
U' L2 B' R' L' D F' R' L' F U F' L' B L R' D' B2 R F R2 F2 R' U F' U F' B' L' D' F2 R' U' F U' F' U' R' F' R2 F2 R'
```

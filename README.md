# cube-rs
[![license: MIT](https://img.shields.io/badge/license-MIT-green)](https://github.com/vcs222/cube-rs/blob/master/LICENSE)

**cube-rs** is a minimalist Rubik's cube scramble generator.

## Install
Build a release version and install it globally:
```
git clone https://github.com/vcs222/cube-rs.git
cd cube-rs
cargo build --release
sudo cp target/release/cube-rs /usr/local/bin
```

## Generate a scramble sequence:
* Open CLI and run `cube-rs` for a default of 25 moves:
```
U2 R' U' B2 D2 U' D' L' B2 D' R B F' R2 F' L2 R' B F2 B' L B' F' B F2 
```
* Specify the length of the sequence:
```
cube-rs [length_of_sequence]
cube-rs 33
```
output:
```
F2 B F' D2 R2 D R2 B2 F' U' L F D' U2 R2 F' D' F' B U' D' R2 F U2 L' U' D' R' B' F' U B2 D2
```

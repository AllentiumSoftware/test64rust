/*
test64.rs
This tests for a 64 bit or 32 bit platform using the Rust programming language.
Written by Darcy Allen, March 4, 2020.
--------
This is "Truly Open Source".
No copyright. No license. No agreements.
Yes you can copy this. Yes you can give it away. Yes you can make money with it.
Truly Open Source is about caring, sharing, and giving freely to the world.
You are free to copy this source code and use it anyway that you want to.
You do not have to redistribute your modifications to anyone.
You do not have to include this text in your copied source code if you don't want to.
You have the freedom to use this truly open source in commercial software,
to earn money from sales of the binary runtime files from this source and/or from the
source code itself or portions of this source code, and you have the right to convert this
into your own closed source if you want to.
You should not expect bug fixes nor enhancements for this source code and the
author or authors of the source code shall not be held liable for any damages whatsoever.
Similarily, you are not expected to share your bug fixes nor your enhancements to where you
got this source code from.
--------
Instructions
If you need to install Rust:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    when prompted, type 1 and hit enter (the default is safe and ok)
    export PATH=$PATH:$HOME/.cargo/bin

Build:
    rustc test64.rs

Run:
    ./test64
*/
use std::mem;
fn main(){
    let test:isize = 123;
    let size_of_test = mem::size_of_val(&test);
    println!("test value {} takes up {} bytes therefore you are on a {}-bit platform", test, size_of_test, size_of_test * 8);
}
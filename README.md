# rust-wc-cli

## Description
Quick Rust based CLI command that is meant to mimic the output from the "wc" command.

Note: This program has limited functionality, it only accepts the following arguments `-c`, `-l`, `-m` and `-w`.

## Motivation 
This small project is meant for me to get my hands on Rust and experiment with the language.

## How to run the project?
- Download [Rust](https://www.rust-lang.org/learn/get-started) 
- In your command line, run `cargo run -- test.txt`

Note: This command uses the file stored at the root directory. You can test this command with your own file/input as well.

## For the future?
If I had some time, I would
- work on cleaning up the function to make it more modular (separating logic into functions)
- perform the file read once and store the word, line, and byte count 
- use one type of I/O file reading (improve perfomance)



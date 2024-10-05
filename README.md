# yayachip8rsemu

![](/preview.gif)

Yet another `yet another chip8 rust emulator` that I created for fun,
because there aren't enough of them already. Well to be precise I used
the word emulator because it's a popular thing in the EmuDev community
but it's an interpreter since a CHIP-8 CPU doesn't exist. If you're
writing a CHIP-8 interpreter on your own this project might be useful
for you to compare it with your implementation when you will come
across a bug or something, especially since I put some focus on having
a good disassembly, debugger, etc.

## Features
- A simple debugger
- Disassembler
- "Time travel" through snapshots

## How to run
Compile and run with the following command:
```cargo run --bin emulator -- --file some_game_path --debug-mode```

There's also a disassembler that you can compile with:
```cargo rustc --bin disassembler```

To see all the possible arguments add `--help` to the end.

## Keyboard
The keyboard mapping is hardcoded in the code:
```rust
    let keyboard_key_chip8_key_pairs = [
        (KeyCode::Key1, 0x1),
        (KeyCode::Key2, 0x2),
        (KeyCode::Key3, 0x3),
        (KeyCode::Key4, 0xc),
        (KeyCode::Q, 0x4),
        (KeyCode::W, 0x5),
        (KeyCode::E, 0x6),
        (KeyCode::R, 0xd),
        (KeyCode::A, 0x7),
        (KeyCode::S, 0x8),
        (KeyCode::D, 0x9),
        (KeyCode::F, 0xe),
        (KeyCode::Z, 0xa),
        (KeyCode::X, 0x0),
        (KeyCode::C, 0xb),
        (KeyCode::V, 0xf),
    ];
```

## TODO
TODO:
- add an option to choose implementation quirks (https://github.com/Timendus/chip8-test-suite#quirks-test)

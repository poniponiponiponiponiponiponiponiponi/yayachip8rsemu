![](/preview.gif)

Yet another `yet another chip8 rust emulator` that I created to play around
with rust, get the basic file structure right for my future rust projects, etc.
If you're writing a chip8 emulator for yourself this project might be useful
for you to compare it with your own when you will come across a bug or
something, especially since I put a lot of weight on having good disassembly,
debugging, etc.

Compile and run with the command:
```cargo run --bin emulator -- --file some_game_path --debug-mode```
There's also a disassembler that you can compile with:
```cargo rustc --bin disassembler```

TODO:
- add more tests
- add an option to choose implementation quirks (https://github.com/Timendus/chip8-test-suite#quirks-test)
- add an option to go back in time (it's not as hard to implement as it might seem)

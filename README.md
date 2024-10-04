# yayachip8rsemu

![](/preview.gif)

Yet another `yet another chip8 rust emulator` that I created for fun,
because there aren't enough of them already. If you're writing a chip8
emulator on your own this project might be useful for you to compare
it with your implementation when you will come across a bug or
something, especially since I put some focus on having a good
disassembly, debugger, etc.

Compile and run with the following command:
```cargo run --bin emulator -- --file some_game_path --debug-mode```
There's also a disassembler that you can compile with:
```cargo rustc --bin disassembler```

TODO:
- add an option to choose implementation quirks (https://github.com/Timendus/chip8-test-suite#quirks-test)
- add an option to go back in time (it's not as hard to implement as it might seem)

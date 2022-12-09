# hangman
My implementation of the classic Hangman game in Rust.
I took some inspiration from Ryan Gordon: [Hangman](https://github.com/ryangordon11235/Hangman), however my implementation invloves more advanced concepts in the Rust programming language.

Differences between this code and [Hangman](https://github.com/ryangordon11235/Hangman):
- All of my code is #![no_std] and instead links just with `libc`.
- Therefore, none of my code uses heap allocations (`libc` might be doing weird stuff though).
- My code does not use any of Ryan's but does use his word list, not in `JSON` and without heap allocations (it's compiled in). 
- Lastly, our actual game is different, especially my TUI uses [`ANSI`](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797) escape codes.

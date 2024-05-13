# Hack Assembler

This is an assembler for the Hack computer, which I've written for the [Nand2Tetris course](http://nand2tetris.org/). It translates assembly code written for the Hack computer into machine code that the Hack hardware platform can execute.

## Help Output

```shell
This is an assembler for the Hack computer, part of the Nand2Tetris course

Usage: hack_assembler.exe [OPTIONS]

Options:
  -f, --file [<FILE>]  Sets the input file
  -h, --help           Print help
  -V, --version        Print version
```


### Build
```shell
git clone
cd hack_assembler
cargo build --release
```
### Run
```shell
./target/release/hack_assembler -f path/to/your/file.asm
```

### Assembly Code

```assembly
// test
(START)
  @i      // Set A-register to address of i (variable or label)
  M=1     // Memory[i] = 1
  @LOOP   // Set A-register to address where LOOP is defined
  0;JMP   // Jump to LOOP

(LOOP)
  @i      // Set A-register to address of i
  M=M+1   // Memory[i] = Memory[i] + 1
  @END    // Set A-register to address where END is defined
  D;JLT   // If D < 0, jump to END

(END)
  @END
  0;JMP   // Infinite loop to END
```
### Machine Code
```binary
0000000000010000
1110111111001000
0000000000000100
1110101010000111
0000000000010000
1111110111001000
0000000000001000
1110001100000100
0000000000001000
1110101010000111
```

### Example
```shell
`target\debug\hack_assembler.exe -f assets/Pong.asm`
Successfully assembled the file: assets/Pong.hack
```
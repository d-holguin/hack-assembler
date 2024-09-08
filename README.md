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
// test.asm Simple counter from 0 to 10
(START)
  @i
  M=0
  @10       // Set A-register to 10 (counter)
  D=A

(LOOP)
  @i        // Load address of i
  M=M+1     // Increment i by 1

  D=D-1     // Decrement the counter in D
  @END      // If counter == 0, jump to END
  D;JEQ     // Jump if D (counter) == 0

  @LOOP     // Jump back to LOOP if counter > 0
  0;JMP

(END)
  @END      // Infinite loop at END
  0;JMP     // Stop the program

```
### Machine Code
```binary
0000000000010000
1110101010001000
0000000000001010
1110110000010000
0000000000010000
1111110111001000
1110001110010000
0000000000001011
1110001100000010
0000000000000100
1110101010000111
0000000000001011
1110101010000111
```

### Example
```shell
`target\debug\hack_assembler.exe -f assets/Pong.asm`
Successfully assembled the file: assets/Pong.hack
```



### Output with Errors

```shell
# error.asm
Error running the Hack Assembler: Assembly encountered the following errors:
  - Invalid instruction on line 2: ^error^(START)
  - Invalid instruction on line 9: ^error^  @i
  - Invalid instruction on line 12: D(invalid)1
  - Invalid instruction on line 22: 0:JUMP
```
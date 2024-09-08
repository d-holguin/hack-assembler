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

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
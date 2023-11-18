# Useless Virtual Machine

This is a register and stack based virtual machine implemented in Rust as an exercise to start learning the language.

**It's a "for fun and learning" project!**

### Virtual Machine

The virtual machine is a register and stack based machine.

Besides configuration flags which are stored in it as well, in practice it has (`*` means configurable):
- `*` 64-bit registers
- a stack of 64-bit values up to `*` entries
- a call stack up to `*` entries
- 1 64-bit instruction pointer
- 1 64-bit stack pointer
- 1 64-bit call stack pointer
- 1 8-bit comparison flag store

### Instruction Set

Each instruction is represented by an 8-bit value, although as of the writing of this section on the README only 6 bits are ever used as we only have 33 instructions implemented.

They can be found at `src/asm.rs` where they will be up to date for sure. But, letting CoPilot create a nice README, these currently are:

| Opcode  | Description                                                                              |
| ------- | ---------------------------------------------------------------------------------------- |
| HALT    | Stops execution                                                                          |
| SET     | `x rb`: Sets `rb` to `x`                                                                 |
| PUSH    | `rb`: Pushes the value of `rb` to the stack                                              |
| PUSHL   | `x`: Pushes `x` to the stack                                                             |
| POP     | `rb`: Pops the top of the stack to `rb`                                                  |
| PUSHRF  | `x`: Saves the value of the first `x` registers to the stack                             |
| POPRF   | `x`: Loads the value of the first `x` registers from the stack                           |
| ADD     | `ra rb`: Adds `ra` and `rb` and stores the result in `rb`                                |
| ADDL    | `x rb`: Adds `x` and `rb` and stores the result in `rb`                                  |
| SUB     | `ra rb`: Subtracts `ra` from `rb` and stores the result in `rb`                          |
| SUBLA   | `x rb`: Subtracts `x` from `rb` and stores the result in `rb`                            |
| SUBLB   | `x rb`: Subtracts `rb` from `x` and stores the result in `rb`                            |
| MUL     | `ra rb`: Multiplies `ra` and `rb` and stores the result in `rb`                          |
| MULL    | `x rb`: Multiplies `x` and `rb` and stores the result in `rb`                            |
| DIV     | `ra rb`: Divides `rb` by `ra` and stores the result in `rb`                              |
| DIVLA   | `x rb`: Divides `rb` by `x` and stores the result in `rb`                                |
| DIVLB   | `x rb`: Divides `x` by `rb` and stores the result in `rb`                                |
| MOD     | `ra rb`: Stores the remainder of `rb` divided by `ra` in `rb`                            |
| INC     | `rb`: Increments `rb` by 1                                                               |
| DEC     | `rb`: Decrements `rb` by 1                                                               |
| CMP     | `ra rb`: Compares `rb` and `ra` and stores the result in `cmp` (e.g., GT if `rb` > `ra`) |
| CMPL    | `x rb`: Compares `rb` and `x` and stores the result in `cmp` (e.g., GT if `rb` > `x`)    |
| JMP     | `addr`: Jumps to `addr`                                                                  |
| JEQ     | `addr`: Jumps to `addr` if `cmp` has EQ                                                  |
| JLT     | `addr`: Jumps to `addr` if `cmp` has LT                                                  |
| JLE     | `addr`: Jumps to `addr` if `cmp` has LE                                                  |
| JGT     | `addr`: Jumps to `addr` if `cmp` has GT                                                  |
| JGE     | `addr`: Jumps to `addr` if `cmp` has GE                                                  |
| JNE     | `addr`: Jumps to `addr` if `cmp` has NE                                                  |
| CALL    | `addr`: Calls the function at `addr` saving the current address in the call stack        |
| RET     | Returns from a function (pops the call stack and jumps to the saved address)             |
| DBGREG  | `rb`: Prints the value of `rb` to stdout for debugging                                   |
| DBGREGS | Prints the values of all registers to stdout for debugging                               |

### Assembly

The "assembly" is a simple text format that can be assembled into bytecode. It's mostly very intuitive:

- Each line is an instruction
- Each instruction has a name and arguments
- Arguments are separated by whitespace
- Arguments can be registers, labels, or literals
- Registers are represented by `rX` where `X` is the register number
- Defining a label is done by writing `label:` in a line by itself
- Referencing a label is done by writing `label` as an argument
- There can be sublabels (e.g. `.sublabel:` below a `label:` gets expanded to `label.sublabel:`) for convenience
- Literals are represented by numbers (e.g. `123`)
- Comments are started by writing `//` and last until the end of the line (they can come after instructions or in lines by themselves)

For example a valid program that calculates the factorial of 5 and prints it to stdout with the `DBGREG` instruction would be:

```
// Factorial of 5
PUSHL	5
CALL	factorial
POP	r0
DBGREG	r0
HALT

factorial:
	POP	r0		// r0 = n <- we'll iterate here
	SET	1	r1	// r1 = 1 <- we'll accumulate multiplications here
	
.loop:
	CMPL	1	r0	// Compare r0 with 0
	JEQ	.end		// When n == 0, jump to the end
	MUL	r0	r1	// r1 = r0 * r1
	DEC	r0		// r0 = r0 - 1
	JMP	.loop		// Make the comparison again

.end:
	PUSH	r1
	RET
```

### Serialization

This is still being implemented, but the `serialize` and `deserialize` functions are already implemented.
They are used to serialize parsed "code" into bytes which can (TBD) be written to disk and then read from them.

Currently "code" can contain four different "atoms", each serialized through:
- OpCodes: 1 byte
- Registers: 1 byte
- Addresses: 8 bytes
- Literals: 8 bytes

Regarding addresses, it just seemed simpler to actually incorporate the address itself on code instead of the label. Using the label could save space if on average they are referenced more than 1 time since we could then represent labels by 1 byte (pointing to a label table with the actual addresses), but this indirection doesn't seem worth it.

### Example programs

There are some example programs in the `tests` folder which are used for integration tests.

The only one that might come close to interesting so far is `tests/recursive_fibonacci.uvm` which was used to test the call stack implementation.

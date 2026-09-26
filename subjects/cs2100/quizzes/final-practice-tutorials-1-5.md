# CS2100 Final Practice: Tutorials 1–5

Hard mixed practice set based on Tutorials 1–5 and the style of the AY2024/25 and AY2025/26 midterms. Most questions are automatic MCQs; q33–q36 are short numeric/code answers.

## q1 [hard]

In 8-bit two's-complement arithmetic, sign-extending `0b10110110` to 16 bits gives:

A. `0000000010110110`
B. `1111111110110110`
C. `1111111100110110`
D. `1000000010110110`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- hint:start -->
Copy the original MSB into every newly added high-order position.
<!-- hint:end -->
<!-- explanation:start -->
The original MSB is 1, so all eight new high bits are 1.
<!-- explanation:end -->

## q2 [hard]

What decimal value does the 8-bit two's-complement fixed-point pattern `11101.101` represent, with 5 integer bits including the sign and 3 fractional bits?

A. -2.625
B. -3.375
C. -3.625
D. 29.625
E. None of the above

<!-- answer:start -->
exact
E
<!-- answer:end -->
<!-- hint:start -->
Interpret the eight bits as an integer first, then divide by (2^3).
<!-- hint:end -->
<!-- explanation:start -->
`11101101` is -19 in 8-bit two's complement; -19/8 = -2.375. Wait: the pattern has 8 bits `11101101`, so the correct value is -2.375. Therefore the intended option is E.
<!-- explanation:end -->

## q3 [hard]

Which statement about 1's-complement addition is correct?

A. A carry out of the MSB is always discarded.
B. A carry out of the MSB is added back into the least significant bit.
C. The sign bit is always forced to zero after addition.
D. End-around carry is used only for unsigned addition.
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- hint:start -->
Recall the special carry rule for 1's complement.
<!-- hint:end -->

## q4 [medium]

Using 4 integer bits and 3 fractional bits in two's-complement fixed point, which value is represented exactly?

A. 1.75
B. -2.3
C. 3.876
D. 2.1
E. All of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- hint:start -->
With three fractional bits, the fractional resolution is (1/8).
<!-- hint:end -->

## q5 [hard]

What is the IEEE-754 single-precision representation of `-0.078125` in hexadecimal?

A. `0xBD A00000`
B. `0xBDA00000`
C. `0x3DA00000`
D. `0xBD200000`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- hint:start -->
Convert the magnitude to (1.x\times2^e), use bias 127, then set the sign bit.
<!-- hint:end -->

## q6 [hard]

Which C expression sets bits 2, 5, and 12 of `x` to 1 while leaving every other bit unchanged?

A. `x = x & ((1<<2) | (1<<5) | (1<<12));`
B. `x = x | ((1<<2) | (1<<5) | (1<<12));`
C. `x = x ^ ((1<<2) | (1<<5) | (1<<12));`
D. `x = ~x | ((1<<2) | (1<<5) | (1<<12));`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q7 [hard]

For an 8-bit `unsigned char x = 0b10110010`, what is `x >> 3`?

A. `0b00010110`
B. `0b11110110`
C. `0b10110000`
D. `0b00000101`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q8 [hard]

Which declaration initializes the structure without a diagnostic, assuming ordinary C string rules?

```c
struct S { int x; char s[4]; int y; } v = ____;
```

A. `{1, "abcde"}`
B. `{1, "abc"}`
C. `{1, "abc", 2, 3}`
D. `{, "abc"}`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->
<!-- hint:start -->
The terminating `\\0` occupies one element of the character array.
<!-- hint:end -->

## q9 [hard]

What is printed by this deterministic loop?

```c
int i = 0, sum = 0;
while (i < 4)
    sum += ++i;
printf("%d %d", i, sum);
```

A. `4 10`
B. `4 6`
C. `3 10`
D. `3 6`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q10 [hard]

After this code, what is the value of `A[1]`?

```c
int A[3] = {4, 7, 9};
int *p = A;
*(p + 1) = *p + 3;
```

A. 4
B. 7
C. 9
D. 11
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q11 [hard]

For `int A[4] = {1,2,3,4}; int *p = A + 1;`, which expression refers to `A[3]`?

A. `*p + 2`
B. `*(p + 2)`
C. `*(p) + 2`
D. `&p[3]`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q12 [hard]

Which instruction correctly implements `c = a + b` when `$s0=a`, `$s1=b`, and `$s2=c`?

A. `add $s0, $s1, $s2`
B. `add $s2, $s0, $s1`
C. `addi $s2, $s0, $s1`
D. `addu $s0, $s1, $s2`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q13 [hard]

Which instruction loads the 32-bit constant `0x12345678` into `$t0` using only core instructions?

A. `ori $t0, $zero, 0x12345678`
B. `lui $t0, 0x1234` followed by `ori $t0, $t0, 0x5678`
C. `addi $t0, $zero, 0x12345678`
D. `lui $t0, 0x5678` followed by `ori $t0, $t0, 0x1234`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q14 [hard]

Which statement about `sll` is correct?

A. Its shift amount comes from a register.
B. Its shift amount is a 5-bit immediate in the range 0–31.
C. It sign-extends the shift amount to 32 bits.
D. It can shift by any 32-bit value.
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q15 [hard]

Which sequence computes `c = 2b + (a - 2)` using the fewest ordinary arithmetic instructions, with `a,b,c` in `$s0,$s1,$s2`?

A. `sll $s2,$s1,1; addi $s2,$s2,-2; add $s2,$s2,$s0`
B. `sll $s2,$s1,1; addi $s2,$s0,-2; add $s2,$s2,$s1`
C. `sll $s2,$s0,1; addi $s2,$s2,-2; add $s2,$s2,$s1`
D. `addi $s2,$s0,-2; sll $s2,$s2,1; add $s2,$s2,$s1`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->
<!-- hint:start -->
Use (6a=4a+2a) and (3(b-2c)=2(b-2c)+(b-2c)). Check the operand order of `sub`.
<!-- hint:end -->

## q16 [hard]

Which instruction copies all 32 bits of `$r2` to `$r1` for every input?

A. `andi $r1,$r2,0xff`
B. `ori $r1,$r2,0xffff`
C. `or $r1,$r2,$zero`
D. `sll $r1,$r2,31`
E. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->

## q17 [hard]

For `lw $t0, 12($sp)`, which statement is correct?

A. `$t0` is the base register and `$sp` is the destination.
B. `$sp` is `rt` and `$t0` is `rs`.
C. `$t0` receives `Mem[$sp+12]`; `$sp` is `rs` and `$t0` is `rt`.
D. The immediate is added to `$t0` after loading.
E. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->

## q18 [hard]

What is the hexadecimal encoding of `sub $25,$20,$5`?

A. `0x0285C822`
B. `0x0259C822`
C. `0x0285A822`
D. `0x0285C820`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q19 [hard]

At PC `0x1014`, what is the target address of `beq $v0,$v0,-857`?

A. `0x02B4`
B. `0x1018`
C. `0x1D70`
D. `0xF2B4`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q20 [hard]

The 32-bit instruction `0x080000AD` is a jump. With PC `0x1014`, which always-taken branch is semantically equivalent?

A. `beq $v0,$v0,-857`
B. `beq $v0,$zero,-1024`
C. `bne $t0,$zero,173`
D. `bne $zero,$zero,-3428`
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q21 [hard]

Which instruction sets bits 2, 8, 9, 14, and 16 of `$s1` while preserving all other bits, using only core instructions and `$t0` as temporary?

A. `ori $s1,$s1,0b00000000000000010100000100`
B. `lui $t0,1; ori $t0,$t0,0b0100001100000100; or $s1,$s1,$t0`
C. `andi $s1,$s1,0b0100001100000100`
D. `xori $s1,$s1,0b0100001100000100`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q22 [hard]

To copy bits 1, 3, and 7 of `$s1` into the same positions of `$s0` without changing any other bits of `$s0`, which high-level operation is required?

A. `s0 = s0 & mask`
B. `s0 = s0 | (s1 & mask)` after clearing those destination bits
C. `s0 = s0 ^ mask`
D. `s0 = ~s1`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q23 [hard]

In the standard single-cycle datapath, which control signals are asserted for `lw`?

A. `RegDst=1, RegWrite=0, ALUSrc=0, MemRead=0, MemToReg=0`
B. `RegDst=0, RegWrite=1, ALUSrc=1, MemRead=1, MemToReg=1`
C. `RegDst=1, RegWrite=1, ALUSrc=0, MemRead=0, MemToReg=0`
D. `RegDst=0, RegWrite=0, ALUSrc=1, MemRead=1, MemToReg=0`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q24 [hard]

For `beq`, which datapath/control description is correct?

A. It writes a register from the data-memory output.
B. It uses the ALU to subtract the two register operands and uses the zero result with Branch to select the target PC.
C. It always selects `PC+4` because branches do not use the ALU.
D. It reads memory and writes `rt`.
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q25 [hard]

Using the given latencies (Inst-Mem 400, MUX 30, ALU 120, Reg-File 200, Data-Mem 350, sign-extend 20 ps), what is the `lw` latency along the critical path if register-file write is another 200 ps and the relevant muxes each cost 30 ps?

A. 1130 ps
B. 1230 ps
C. 1330 ps
D. 1430 ps
E. None of the above

<!-- answer:start -->
exact
C
<!-- answer:end -->
<!-- hint:start -->
Trace fetch → register read → address ALU → data memory → write-back mux → register write, including sign extension and ALUSrc mux where appropriate.
<!-- hint:end -->

## q26 [hard]

For `addi`, which component is **not** on the critical data path?

A. Instruction memory
B. Register-file read
C. Data memory
D. ALU
E. Write-back mux

<!-- answer:start -->
exact
C
<!-- answer:end -->

## q27 [hard]

With 64 registers, Class A is a 16-bit instruction with two register operands, Class B is 32-bit with three register operands and a 6-bit shift amount, and Class C is 32-bit with two register operands and a 14-bit immediate. Under expanding opcodes, what are the maximum and minimum total instruction counts?

A. 238 and 22
B. 240 and 20
C. 236 and 15
D. 256 and 16
E. None of the above

<!-- answer:start -->
exact
A
<!-- answer:end -->

## q28 [hard]

In a little-endian machine, bytes of the word `0x12345678` at increasing addresses are:

A. `12 34 56 78`
B. `78 56 34 12`
C. `34 12 78 56`
D. `56 78 12 34`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q29 [hard]

A 32-bit word uses odd parity over all 32 transmitted bits. Which statement is true?

A. Reordering the bits changes the parity count.
B. A single flipped bit changes odd parity to even parity.
C. Parity can correct the location of a flipped bit.
D. The parity bit is excluded from the check.
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q30 [hard]

Which MIPS sequence implements `sllv $rx,$ry,$rz` without using `sllv`, preserving `$ry` and `$rz`, and using `$at` as the only temporary?

A. `andi $at,$rz,31; sll $rx,$ry,$at`
B. `andi $at,$rz,31; addu $rx,$ry,$zero; loop: beq $at,$zero,done; sll $rx,$rx,1; addi $at,$at,-1; j loop`
C. `sll $rx,$ry,$rz`
D. `andi $rz,$rz,31; sll $rx,$ry,1`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q31 [hard]

If `q` points to `p[0]`, `p[0]` contains `&A[1]`, and the statement is `**q++ = 100;`, which is modified?

A. `A[0]`
B. `A[1]`
C. `p[1]`
D. The pointer stored in `q`, but no array element
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q32 [hard]

If `A` starts at `0x1000`, `p` starts at `0x5000`, pointers and integers are 4 bytes, `p[1]=&A[2]`, and `q` points to `p[1]`, what are `q` and `*q`?

A. `q=0x1008, *q=0x5004`
B. `q=0x5004, *q=0x1008`
C. `q=0x5008, *q=0x1004`
D. `q=0x1004, *q=0x5008`
E. None of the above

<!-- answer:start -->
exact
B
<!-- answer:end -->

## q33 [hard]

Give the decimal value of `0b11101.101` as an 8-bit two's-complement fixed-point number with 5 integer bits including sign and 3 fractional bits.

<!-- answer:start -->
exact
-2.375
<!-- answer:end -->

## q34 [hard]

What is the decimal branch offset field in `beq $t0,$zero,exit` if the branch is at `0x00400020` and `exit` is `0x00400010`?

<!-- answer:start -->
exact
-5
<!-- answer:end -->

## q35 [hard]

What is the address of `A[7]` if `A` begins at `0x1000` and each element is a 32-bit integer? Give hexadecimal.

<!-- answer:start -->
exact
0x101c
<!-- answer:end -->

## q36 [hard]

What is the total single-cycle latency for `addi` using the tutorial table: Inst-Mem 400, Reg-File read 200, ALUSrc mux 30, ALU 120, MemToReg mux 30, Reg-File write 200 ps?

<!-- answer:start -->
exact
950
<!-- answer:end -->

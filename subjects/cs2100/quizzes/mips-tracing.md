# CS2100 MIPS — Tracing, Encoding & Decoding Quiz

Covers registers, R/I/J formats, encoding and decoding, arithmetic/logical, memory, branches/jumps, addressing modes and C-to-MIPS translation. Answers are in the answer/explanation blocks — work each one on paper first. All numeric answers are decimal unless a `0x` prefix or "hex" is stated.

## q1 [easy]

What is the final value of `$t0` after this snippet?

```
addi $t0, $zero, 100
addi $t1, $zero, 7
add  $t0, $t0, $t1
addi $t0, $t0, -3
```

<!-- answer:start -->
numeric 0
104
<!-- answer:end -->
<!-- explanation:start -->
Step by step:

1. `addi $t0, $zero, 100` → `$t0 = 0 + 100 = 100`.
2. `addi $t1, $zero, 7` → `$t1 = 7`.
3. `add $t0, $t0, $t1` → `$t0 = 100 + 7 = 107`.
4. `addi $t0, $t0, -3` → the immediate is 16-bit 2's complement (signed), so this is `107 + (-3) = 104`.

Final `$t0 = 104`. (There is no `subi` instruction — `addi` with a negative immediate is how you subtract.)
<!-- explanation:end -->
<!-- hint:start -->
Remember that `addi`'s immediate is a signed 16-bit value, so a negative immediate subtracts — and execution is strictly sequential.
<!-- hint:end -->

## q2 [easy]

Convert `sub $t0, $t3, $t5` to its 32-bit machine code. Give the answer as exactly 8 hexadecimal digits (no `0x` prefix).

<!-- answer:start -->
exact
016d4022
0x016d4022
<!-- answer:end -->
<!-- explanation:start -->
`sub` is R-format, so `opcode = 0` and the field order is `opcode | $rs | $rt | $rd | shamt | funct`.

- `$rs = $t3 = 11` = `01011`
- `$rt = $t5 = 13` = `01101`
- `$rd = $t0 = 8` = `01000`
- `shamt = 0` = `00000`
- `funct = sub = 34` = `100010`

Concatenate: `000000 01011 01101 01000 00000 100010`
= `0000 0001 0110 1101 0100 0000 0010 0010` = `0x016D4022`.

Sanity check by decoding back: `0000 0001 0110 1101 0100 0000 0010 0010` splits as opcode `000000`, `$rs = 01011` = 11 (`$t3`), `$rt = 01101` = 13 (`$t5`), `$rd = 01000` = 8 (`$t0`), shamt `00000`, funct `100010` = 0x22 (`sub`) — which is exactly `sub $t0, $t3, $t5`.

Note the operand-order trap: the assembly is written `$rd, $rs, $rt` but the encoding is `$rs, $rt, $rd`.
<!-- explanation:end -->
<!-- hint:start -->
Work out the R-format field order (`opcode | rs | rt | rd | shamt | funct`) first, and note the assembly operand order is `rd, rs, rt`.
<!-- hint:end -->

## q3 [easy]

In MIPS, which single register always reads as zero and cannot be written? Give the conventional name.

<!-- answer:start -->
exact
$zero
zero
$zero (register 0)
register 0
$0
r0
<!-- answer:end -->
<!-- explanation:start -->
`$zero` (register 0) is hard-wired to the constant value 0 and cannot be written. This makes it useful for register-to-register moves (`add $s0, $s1, $zero`) and as the "never equal" comparison operand. `$at` (1) is reserved for the assembler; `$k0`/`$k1` (26-27) are reserved for the OS.
<!-- explanation:end -->
<!-- hint:start -->
Recall which register number is hard-wired to the constant 0 by the ISA, and what it is conventionally called.
<!-- hint:end -->

## q4 [easy]

Which MIPS instruction format does each of these use — R, I or J?

1. `add`
2. `lw`
3. `beq`
4. `j`
5. `sll`
6. `ori`

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
1. `add` — **R** (two source registers, one destination; opcode 0, funct 0x20)
2. `lw` — **I** (base register + 16-bit offset; opcode 0x23)
3. `beq` — **I** (two registers + 16-bit PC-relative immediate; opcode 0x04)
4. `j` — **J** (6-bit opcode + 26-bit immediate; opcode 0x02)
5. `sll` — **R**, using the `shamt` variant: `opcode | $rs(=0) | $rt | $rd | shamt | funct(=0)`
6. `ori` — **I** (destination `$rt`, source `$rs`, 16-bit unsigned immediate; opcode 0x0D)

Summary: R = `opcode|rs|rt|rd|shamt|funct`; I = `opcode|rs|rt|immediate`; J = `opcode|immediate`. Branches and load/store are I-format; shifts are R-format; only `j` (and out-of-scope `jal`) is J-format.
<!-- explanation:end -->
<!-- hint:start -->
Ask whether the instruction needs an immediate or constant field: that is what separates the three formats.
<!-- hint:end -->

## q5 [easy]

In an I-format instruction such as `addi`, the 16-bit immediate is sign-extended to 32 bits. Give the full 32-bit pattern that the immediate is extended to for the hexadecimal field `0xFFCE`. Give the answer as 8 hexadecimal digits with a `0x` prefix.

<!-- answer:start -->
exact
0xffffffce
ffffffce
<!-- answer:end -->
<!-- explanation:start -->
`0xFFCE` = `1111 1111 1100 1110`. The most significant bit is 1, so it is a negative 16-bit 2's complement value. Sign extension duplicates the MSB eight times to fill the upper 16 bits:

`1111 1111 1111 1111 1111 1111 1100 1110` = `0xFFFFFFCE`.

As a signed value this is -50, because `65536 - 50 = 65486 = 0xFFCE`. Contrast with a **logical** immediate (`andi`, `ori`, `xori`), which is bit-extended: `0xFFCE` would become `0x0000FFCE` (append zeros).
<!-- explanation:end -->
<!-- hint:start -->
Sign extension copies the most significant bit of the 16-bit field into every upper bit — check whether that bit is 0 or 1.
<!-- hint:end -->

## q6 [easy]

The 32-bit word `0x89ABCDEF` is stored at word-aligned address `0x1000` on a little-endian machine. What is the byte held at address `0x1001`? Give the answer as two hexadecimal digits with a `0x` prefix.

<!-- answer:start -->
exact
0xcd
cd
<!-- answer:end -->
<!-- explanation:start -->
Little-endian stores the **least** significant byte at the lowest address:

| Address | Byte |
|----|----|
| 0x1000 | 0xEF (least significant) |
| 0x1001 | **0xCD** |
| 0x1002 | 0xAB |
| 0x1003 | 0x89 (most significant) |

So `0x1001` holds `0xCD`. Note `lb $t0, 0($s0)` would load `0x000000EF`; `lb $t0, 1($s0)` loads `0x000000CD` — `lb` always loads into the **lower** byte and zeroes the upper three bytes.
<!-- explanation:end -->
<!-- hint:start -->
Little-endian stores the least significant byte at the lowest address; unpack the word into bytes from there.
<!-- hint:end -->

## q7 [easy]

What is the final value of `$t0` after this snippet?

```
addi $t0, $zero, 7
addi $t1, $zero, 3
beq  $t0, $t1, Skip
addi $t0, $t0, 5
Skip:
```

<!-- answer:start -->
numeric 0
12
<!-- answer:end -->
<!-- explanation:start -->
1. `$t0 = 7`, `$t1 = 3`.
2. `beq $t0, $t1, Skip` compares 7 and 3 — **not equal**, so the branch is **not taken** and execution falls through to the next instruction.
3. `addi $t0, $t0, 5` → `$t0 = 7 + 5 = 12`.
4. `Skip:` is only a label, not an instruction; there is nothing after it, so execution ends.

Final `$t0 = 12`. The lesson: a `beq` that is not taken costs one instruction but changes no register. If the snippet had used `bne`, `$t0` would have stayed 7.
<!-- explanation:end -->
<!-- hint:start -->
A `beq` that is not taken simply falls through to the next instruction and changes no register — trace the comparison first.
<!-- hint:end -->

## q8 [medium]

What are the final values of `$t0` and `$s1` after this loop?

```
      addi $t0, $zero, 2
      addi $s1, $zero, 4
Loop: sll  $t0, $t0, 1
      addi $s1, $s1, -1
      bne  $s1, $zero, Loop
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
State table (one row per iteration):

| Check | `$t0` after `sll` | `$s1` after `addi` | `bne` taken? |
|----|----|----|----|
| before loop | 2 | 4 | — |
| iteration 1 | 4 | 3 | yes (3 ≠ 0) |
| iteration 2 | 8 | 2 | yes |
| iteration 3 | 16 | 1 | yes |
| iteration 4 | 32 | 0 | **no** (0 == 0) → exit |

Final: **`$t0 = 32`** (which is `0x20`) and **`$s1 = 0`**.

Extra checks: the loop body ran 4 times; `bne` was executed 4 times and branched 3 times; `sll $t0,$t0,1` is multiplication by 2 each time, so `$t0` went 2 → 4 → 8 → 16 → 32.
<!-- explanation:end -->
<!-- hint:start -->
Track the loop counter and the shifting value through each iteration, and note when the `bne` finally stops branching.
<!-- hint:end -->

## q9 [medium]

A word-aligned integer array `A` starts at `0x10010000` and contains `{4, 5, 6, 7, 8}`. What are the final values of `$t8`, `$t1` and `$t9`?

```
      add  $t0, $zero, $zero
      addi $t0, $zero, 0x10010000
      add  $t8, $zero, $zero
      add  $t1, $zero, $zero
      addi $t9, $zero, 20
Loop: beq  $t1, $t9, Done
      lw   $t2, 0($t0)
      add  $t8, $t8, $t2
      addi $t0, $t0, 4
      addi $t1, $t1, 4
      j    Loop
Done:
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`$t0` walks the array by 4 bytes per element; `$t1` counts bytes consumed and stops at 20 (= 5 elements × 4 bytes).

| `$t1` at `beq` | `MEM[$t0]` | `$t8` after add | `$t0` after addi | `$t1` after addi |
|----|----|----|----|----|
| 0 | 4 | 4 | 0x10010004 | 4 |
| 4 | 5 | 9 | 0x10010008 | 8 |
| 8 | 6 | 15 | 0x1001000C | 12 |
| 12 | 7 | 22 | 0x10010010 | 16 |
| 16 | 8 | 30 | 0x10010014 | 20 |
| 20 | — | `beq` taken to `Done` | — | — |

Final: **`$t8 = 30`** (the sum 4+5+6+7+8), **`$t1 = 20`**, **`$t9 = 20`** (unchanged). `$t0 = 0x10010014` and `$t2 = 8`.
<!-- explanation:end -->
<!-- hint:start -->
Watch what each register is counting: one is a byte address, another stops at bytes consumed, so decide the units and exit condition first.
<!-- hint:end -->

## q10 [medium]

Execute this code and give the final contents of memory word `0x2000`. Give the answer as 8 hexadecimal digits with a `0x` prefix.

```
addi $t0, $zero, 0x2000
addi $t1, $zero, 5
sw   $t1, 0($t0)
lb   $t2, 2($t0)
addi $t3, $zero, -1
sb   $t3, 2($t0)
```

<!-- answer:start -->
exact
0x00ff0005
00ff0005
<!-- answer:end -->
<!-- explanation:start -->
1. `$t0 = 0x2000`, `$t1 = 5`.
2. `sw $t1, 0($t0)` stores the **whole word** `0x00000005` at `0x2000`. Little-endian byte layout: `0x2000 = 0x05`, `0x2001 = 0x00`, `0x2002 = 0x00`, `0x2003 = 0x00`.
3. `lb $t2, 2($t0)` loads the byte at `0x2002`, which is `0x00`, into the **lower** byte of `$t2` and zeroes the upper 3 bytes → `$t2 = 0x00000000`.
4. `$t3 = -1` = `0xFFFFFFFF`.
5. `sb $t3, 2($t0)` stores only the **lower byte** of `$t3`, i.e. `0xFF`, at `0x2002`.

Memory word at `0x2000` is now `0x00 FF 00 05` (bytes at 0x2003/0x2002/0x2001/0x2000) = **`0x00FF0005`**. `sb`/`lb` need no alignment, so offset 2 is legal; `sw` at `0x2000` is legal because the address is a multiple of 4.
<!-- explanation:end -->
<!-- hint:start -->
Track the little-endian byte layout after the `sw`, then check which bytes the subsequent `lb` and `sb` actually touch.
<!-- hint:end -->

## q11 [medium]

Execute this snippet and give the final value of `$t2`. The base register `$t0 = 0x1000`. Give the answer as 8 hexadecimal digits with a `0x` prefix.

```
addi $t0, $zero, 0x1000
lui  $t1, 0x1122
ori  $t1, $t1, 0x3344
sw   $t1, 0($t0)
addi $t2, $zero, 0
lb   $t3, 1($t0)
add  $t2, $t2, $t3
lb   $t4, 3($t0)
sll  $t4, $t4, 8
add  $t2, $t2, $t4
```

<!-- answer:start -->
exact
0x00001133
00001133
<!-- answer:end -->
<!-- explanation:start -->
1. `lui $t1, 0x1122` → `$t1 = 0x11220000` (upper 16 bits set, lower 16 zeroed).
2. `ori $t1, $t1, 0x3344` → `$t1 = 0x11223344` (logical immediate is bit-extended, so `0x00003344` is OR-ed in).
3. `sw $t1, 0($t0)` stores `0x11223344` at `0x1000`. Little-endian bytes: `0x1000 = 0x44`, `0x1001 = 0x33`, `0x1002 = 0x22`, `0x1003 = 0x11`.
4. `$t2 = 0`. `lb $t3, 1($t0)` → `$t3 = 0x00000033`.
5. `add $t2, $t2, $t3` → `$t2 = 0x00000033`.
6. `lb $t4, 3($t0)` → `$t4 = 0x00000011`; `sll $t4, $t4, 8` → `$t4 = 0x00001100`.
7. `add $t2, $t2, $t4` → `0x00000033 + 0x00001100 = 0x00001133`.

Final `$t2 = 0x00001133` (decimal 4403). The two loaded bytes are recombined but shifted relative to the original word, which is exactly the byte-level bookkeeping this question tests.
<!-- explanation:end -->
<!-- hint:start -->
Build the constant in `$t1` first, then remember how `lb` zero-extends a byte before the adds and shift recombine them.
<!-- hint:end -->

## q12 [medium]

Convert `slt $t0, $s1, $t2` to its 32-bit machine code. Give the answer as exactly 8 hexadecimal digits (no `0x` prefix).

<!-- answer:start -->
exact
022a402a
0x022a402a
<!-- answer:end -->
<!-- explanation:start -->
`slt` is R-format: `opcode | $rs | $rt | $rd | shamt | funct`.

- `opcode = 0` = `000000`
- `$rs = $s1 = 17` = `10001`
- `$rt = $t2 = 10` = `01010`
- `$rd = $t0 = 8` = `01000`
- `shamt = 0` = `00000`
- `funct = 0x2A = 42` = `101010`

Concatenate: `000000 10001 01010 01000 00000 101010`
= `0000 0010 0010 1010 0100 0000 0010 1010` = `0x022A402A`.

`slt` sets `$rd` to 1 when `$rs < $rt`, else 0. It is the real instruction behind the pseudo-branches `blt`, `bgt`, `ble` and `bge`, each of which expands to `slt` plus a `beq`/`bne`.
<!-- explanation:end -->
<!-- hint:start -->
Use the R-format field order and the register numbers; `slt` is an R-format instruction with its own `funct` value.
<!-- hint:end -->

## q13 [medium]

A `beq` instruction sits at address `0x00400018` and its target label `Loop` is at address `0x003FFFA0`. What is the value of the 16-bit immediate field of the encoded instruction? Give the answer as a decimal integer (include the sign if negative).

<!-- answer:start -->
numeric 0
-31
<!-- answer:end -->
<!-- explanation:start -->
Use the branch-target formula:

`$PC' = ($PC + 4) + (immediate × 4)`

- `$PC = 0x00400018`, so `$PC + 4 = 0x0040001C`.
- `$PC' = 0x003FFFA0`.
- `0x0040001C - 0x003FFFA0 = 0x7C = 124` bytes.
- `immediate = 124 / 4 = 31`.

Because the target is **before** the current instruction (the branch goes backwards), the immediate is **negative**: **-31**.

Diagram check: box from the line below the branch (`$PC+4`) up to the line above `Loop` and count the instructions inside — 31 of them, sign negative for a backward target. The immediate is a signed 16-bit 2's complement value, so -31 is well within range (-32768..32767).
<!-- explanation:end -->
<!-- hint:start -->
Branch displacements are PC-relative and counted in words from `$PC + 4`; find the byte difference, divide by 4, and check the direction.
<!-- hint:end -->

## q14 [medium]

A `j` instruction sits at address `0x00400020` and jumps to `Loop` at address `0x00400004`. Give the 26-bit immediate field as a 26-bit binary string (spaces optional).

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Pseudo-direct addressing uses `$PC' = (($PC + 4) & 0xF0000000) | (immediate × 4)`. Reverse it:

1. Target `0x00400004` = `0000 0000 0100 0000 0000 0000 0000 0100`. Its **top 4 bits are `0000`** — these are the bits that will be supplied by `$PC+4`, not stored.
2. `$PC + 4` = `0x00400024` = `0000 0000 0100 0000 0000 0000 0010 0100`. Its top 4 bits are also `0000`, so the top nibbles match and the jump is a legal target within the same 256 MB boundary.
3. The lower 28 bits of the target are `0x00400004`, and since `immediate × 4` must equal that, `immediate = 0x00400004 / 4 = 0x00100001` = 1,048,577 decimal.
4. In 26 bits: `0x00100001` = `00 0001 0000 0000 0000 0000 0001` = **`00000100000000000000000001`**.

Sanity check: `(0x00400024 & 0xF0000000) | (0x00100001 × 4) = 0x00000000 | 0x00400004 = 0x00400004`. Correct.

The full instruction would encode as `000010` followed by the immediate = `00001000000100000000000000000001` = `0x08100001`.
<!-- explanation:end -->
<!-- hint:start -->
J-format pseudo-direct addressing takes the top 4 bits from `$PC + 4` and stores the low 28 bits shifted right by 2.
<!-- hint:end -->

## q15 [medium]

A student writes `subi $t0, $t1, 4` to compute `$t0 = $t1 - 4`. Explain why this instruction cannot be encoded, state which real MIPS instruction is used instead, and list the smallest set of real instructions that performs the computation.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`subi` **does not exist** in MIPS. There is deliberately no immediate variant of `sub`: `addi`'s immediate is a 16-bit 2's complement value and therefore may be negative, so subtraction by a constant is expressible as addition of a negative constant. This follows design principle #1, "keep the instruction set small" (the same reason there is no `nori`).

The replacement is a **single real instruction**:

```
addi $t0, $t1, -4     # $t0 = $t1 + (-4) = $t1 - 4
```

Check the encoding: `addi` opcode = 8 (`001000`), `$rs = $t1 = 9` (`01001`), `$rt = $t0 = 8` (`01000`), immediate = `-4` = `0xFFFC` = `1111111111111100`. So `001000 01001 01000 1111111111111100` = `0x2128FFFC`.

Also note the I-format operand-order trap: the assembly is `addi $rt, $rs, immediate`, and the encoding is `opcode | $rs | $rt | immediate`.
<!-- explanation:end -->
<!-- hint:start -->
Remember that there is no immediate form of `sub`, but `addi` accepts a negative immediate — so ask how few instructions are really needed.
<!-- hint:end -->

## q16 [medium]

Trace this loop and give the final value of `$t0`.

```
      add  $t0, $zero, $zero
      addi $t1, $zero, 3
Loop: add  $t0, $t0, $t1
      bge  $t0, 10, Done
      addi $t1, $t1, 3
      j    Loop
Done:
```

(`bge` is a pseudo-instruction that expands to `slt` + `beq`.)

<!-- answer:start -->
numeric 0
18
<!-- answer:end -->
<!-- explanation:start -->
`bge $t0,10,Done` means "branch to `Done` when `$t0 >= 10`".

| Check | `$t0` after `add` | `$t1` | `bge` taken? |
|----|----|----|----|
| before loop | 0 | 3 | — |
| iteration 1 | 3 | 3 | no (3 < 10) → `addi $t1 = 6` |
| iteration 2 | 9 | 6 | no (9 < 10) → `addi $t1 = 9` |
| iteration 3 | 18 | 9 | **yes** (18 >= 10) → `Done` |

Final `$t0 = 18`, `$t1 = 9`. The loop body ran 3 times and the branch was taken once. `$t0` overshoots 10 because the test happens only after `$t1` (3, then 6, then 9) has been fully added.
<!-- explanation:end -->
<!-- hint:start -->
`bge` is a pseudo-instruction expanding to `slt` plus a branch; trace it as a loop whose test happens after the addition.
<!-- hint:end -->

## q17 [medium]

State the **exact** signed value range of the 16-bit immediate field of a `beq` or `bne` instruction (in words, as the field is stored). Give the two endpoints, smallest first.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The immediate field is **16 bits and signed (2's complement)**, so the stored field ranges from **-32768 to 32767**, i.e. `-2^15` to `2^15 - 1`.

How far that reaches: MIPS instructions are word-aligned, so the low 2 bits of the offset are always 0 and the field is used as a **word count** (the hardware multiplies by 4). The reach is therefore **±2^15 words = ±2^17 bytes = ±32768 words / ±131072 bytes** from `$PC + 4`.

Common trap: do not say ±2^15 bytes — that would be true only if the field were used as a raw byte offset, which MIPS deliberately avoids in order to branch 4 times farther.
<!-- explanation:end -->
<!-- hint:start -->
The branch immediate is a signed 16-bit field measured in words, not bytes — work out the limits in words first.
<!-- hint:end -->

## q18 [medium]

Write the MIPS assembly that translates this C code. `n`, `count` and `sum` are in `$s0`, `$s1` and `$s2` respectively, and the result must be left in `$s2`. Do not use any pseudo-instruction other than `blt`/`bge` if you wish.

```
sum = 0;
if (n < 0) {
  count = count + 1;
} else {
  sum = n;
}
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
One correct inversion-based translation:

```
      add  $s2, $zero, $zero    # sum = 0
      addi $t0, $zero, 0
      slt  $t0, $s0, $t0        # $t0 = (n < 0)
      beq  $t0, $zero, Else     # if NOT (n < 0), go to else branch
      addi $s1, $s1, 1          # count = count + 1
      j    Exit
Else: add  $s2, $s0, $zero      # sum = n
Exit:
```

Equivalent forms are acceptable, for example using the pseudo-instruction `bge`:

```
      add  $s2, $zero, $zero    # sum = 0
      bge  $s0, $zero, Else     # if n >= 0, go to else branch
      addi $s1, $s1, 1          # count = count + 1
      j    Exit
Else: add  $s2, $s0, $zero      # sum = n
Exit:
```

Marking notes: `sum = 0` must be done before the branch; the condition must be **inverted** for the "then" body (jump over the body when false); both branches must converge at `Exit`; `sum = n` is a register-to-register copy, correctly written as `add $s2, $s0, $zero` (or `addi $s2, $s0, 0`, or the pseudo `move $s2, $s0`).
<!-- explanation:end -->
<!-- hint:start -->
For an if/else, invert the condition so the then-body is skipped when false, and make both arms converge at one exit label.
<!-- hint:end -->

## q19 [hard]

This MIPS fragment contains exactly three errors, each of which violates a rule stated in the lecture notes. Identify all three and state the rule each one breaks.

```
      subi $t0, $t1, 4
      lw   $t2, 6($s0)
      addi $t3, $zero, 70000
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
1. **`subi` does not exist.** There is no immediate variant of `sub` (design principle #1: keep the instruction set small). A correctly encoded replacement is `addi $t0, $t1, -4`, because `addi`'s immediate is a signed 16-bit value.
2. **`lw $t2, 6($s0)` is misaligned.** `lw`/`sw` require the effective address `$rs + offset` to be a multiple of 4 (word-aligned); `$s0 + 6` is not (unless `$s0` is oddly aligned). Legal fixes: use `lb`/`sb` (no alignment requirement), or correct the offset to a multiple of 4 (`0($s0)` or `4($s0)`). The pseudo-instruction `ulw $t2, 6($s0)` would handle the unaligned access, but pseudo-instructions are generally not allowed in assessments.
3. **`addi $t3, $zero, 70000` overflows the immediate field.** The arithmetic immediate is 16-bit 2's complement, with range -32768 to 32767; 70000 is outside it, so the instruction cannot be encoded (the assembler would reject it). The large-constant procedure must be used instead: `lui $t3, 1` gives `$t3 = 0x00010000` (= 65536) and `ori $t3, $t3, 0x1170` (0x1170 = 4464) gives `0x00011170` = 70000. Note `ori` is required rather than `addi` for the lower half, because `ori`'s immediate is unsigned.

(Not an error, but worth noting: `$s0` as a base register is legal; there is no rule reserving it.)
<!-- explanation:end -->
<!-- hint:start -->
Check each line separately: one uses an opcode that does not exist, one violates an alignment rule, one exceeds an immediate field's range.
<!-- hint:end -->

## q20 [hard]

A `beq $s3, $s4, Loop` instruction is located at address `0x00400008` and `Loop` is at `0x003FFFF4`. Give (a) the value of the 16-bit immediate field in decimal, and (b) the complete machine-code encoding as 8 hex digits.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
**(a)** `$PC' = ($PC + 4) + (immediate × 4)`:

- `$PC + 4 = 0x00400008 + 4 = 0x0040000C`
- `$PC' = 0x003FFFF4`
- difference = `0x003FFFF4 - 0x0040000C = -0x18 = -24` bytes
- `immediate = -24 / 4 = -6`

**(b)** Fields: `opcode(beq) = 4 = 000100`; `$rs = $s3 = 19 = 10011`; `$rt = $s4 = 20 = 10100`; `immediate = -6` = signed 16-bit `1111111111111010` (`0xFFFA`, since `65536 - 6 = 65530 = 0xFFFA`).

Concatenate: `000100 10011 10100 1111111111111010`
= `0001 0010 0111 0100 1111 1111 1111 1010` = **`0x1274FFFA`**.

Verify: `($PC+4) + (imm×4) = 0x0040000C + (-6 × 4) = 0x0040000C - 0x18 = 0x003FFFF4`. Correct.
<!-- explanation:end -->
<!-- hint:start -->
Handle part (a) with the PC-relative word offset, then lay out the I-format fields for part (b), sign-extending the immediate.
<!-- hint:end -->

## q21 [hard]

Decode the machine-code word `0x2128FFFC` back into MIPS assembly. State the instruction, its operands (using `$` register names), and the decimal value of the immediate.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
First convert to binary and split at the I-format field boundaries:

`0x2128FFFC` = `0010 0001 0010 1000 1111 1111 1111 1100`

| Field | Bits | Value |
|----|----|----|
| `opcode` | `001000` | 8 = **`addi`** |
| `$rs` | `01001` | 9 = `$t1` |
| `$rt` | `01000` | 8 = `$t0` |
| `immediate` | `1111111111111100` | `0xFFFC` = **-4** (signed) |

The instruction is **`addi $t0, $t1, -4`**, i.e. `$t0 = $t1 + (-4) = $t1 - 4`.

Immediate conversion: `0xFFFC` has MSB 1, so it is negative: `0xFFFC - 0x10000 = 65532 - 65536 = -4`. Remember that the opcode must be resolved **first** — only then do you know it is I-format and that the last 16 bits are a signed immediate rather than, say, `$rd | shamt | funct`.
<!-- explanation:end -->
<!-- hint:start -->
Read the opcode first to decide the format, then split the remaining bits accordingly — for I-format the last 16 bits are a signed immediate.
<!-- hint:end -->

## q22 [hard]

Decode the machine-code word `0x00A92020` back into MIPS assembly. State the instruction and its three register operands, and say what value the destination register receives.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`0x00A92020` = `0000 0000 1010 1001 0010 0000 0010 0000`.

The top 6 bits are `000000`, so this is **R-format** and the split is `opcode | $rs | $rt | $rd | shamt | funct`:

| Field | Bits (1-indexed) | Bit string | Value |
|----|----|----|----|
| `opcode` | 1-6 | `000000` | 0 → R-format |
| `$rs` | 7-11 | `00101` | 5 = `$a1` |
| `$rt` | 12-16 | `01001` | 9 = `$t1` |
| `$rd` | 17-21 | `00100` | 4 = `$a0` |
| `shamt` | 22-26 | `00000` | 0 (not a shift) |
| `funct` | 27-32 | `100000` | 0x20 = 32 = **`add`** |

The instruction is **`add $a0, $a1, $t1`**: `$a0` receives `$a1 + $t1`.

Double-check by re-encoding: `000000 00101 01001 00100 00000 100000` = `0000 0000 1010 1001 0010 0000 0010 0000` = `0x00A92020`. Correct.

Two traps this question tests: (1) the field order is `$rs, $rt, $rd` while the assembly is written `$rd, $rs, $rt`; (2) `$rs = 5` is `$a1`, not `$t2` — `$t2` is 10 = `01010`, which would give `0x01492020`. Also note `funct = 0` with `$rs = 0` would mean `sll`, so the `funct` field must always be checked.
<!-- explanation:end -->
<!-- hint:start -->
The top opcode says R-format, so use `opcode | rs | rt | rd | shamt | funct`; check `funct` to name the operation.
<!-- hint:end -->

## q23 [hard]

Give the complete MIPS instruction sequence (real instructions only) that loads the 32-bit constant `0x1234A5C0` into `$t0`. Then state the hexadecimal immediate value used by each instruction.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`0x1234A5C0` cannot be produced by a single `addi` or `ori`: the immediate field is only 16 bits. Two real instructions suffice:

```
lui $t0, 0x1234       # $t0 = 0x12340000
ori $t0, $t0, 0xA5C0  # $t0 = 0x1234A5C0
```

Immediate values: `lui` takes `0x1234` (the upper 16 bits); `ori` takes `0xA5C0` (the lower 16 bits).

Why this works: `lui` sets the upper 16 bits to the given constant and **clears the lower 16 bits to 0**; `ori` then bit-ORs in the lower half. `ori`'s immediate is **unsigned** (bit-extended), which is essential here — `0xA5C0` (42432) is beyond the signed range and would sign-extend wrongly under `addi`. `lui` is a **real** instruction, not a pseudo-instruction.

The three-instruction alternative follows the same idea: `ori $t0,$zero,0x1234`; `sll $t0,$t0,16`; `ori $t0,$t0,0xA5C0`. Full credit for either, provided no pseudo-instruction is used.
<!-- explanation:end -->
<!-- hint:start -->
A 32-bit constant needs two steps: one for the upper half and one for the lower half, with the lower half's immediate treated as unsigned.
<!-- hint:end -->

## q24 [hard]

Translate this C code into MIPS. Use `$s0 = count` and `$s1 = i`. You may use pseudo-instructions. Indicate clearly how you form the loop test and where the loop exits.

```
count = 0;
for (i = 0; i < 8; i++) {
  count = count + 2;
}
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
A correct translation (the constant 8 must live in a register because there is no compare-with-immediate branch):

```
      add  $s0, $zero, $zero   # count = 0
      add  $s1, $zero, $zero   # i = 0
      addi $t0, $zero, 8       # $t0 = 8  (loop bound)
Loop: beq  $s1, $t0, Exit      # exit when i == 8
      addi $s0, $s0, 2         # count = count + 2
      addi $s1, $s1, 1         # i++
      j    Loop
Exit:
```

Acceptable variants:

- Test first with `slt $t1, $s1, $t0` then `bne $t1, $zero, Body`, and `j Loop` at the end.
- Use the pseudo-instruction `bge $s1, $t0, Exit` in place of the `beq`, which expands to `slt $at, $s1, $t0` + `beq $at, $zero, Exit`.
- Increment `count` by 2 either with `addi $s0, $s0, 2` or as two `addi $s0, $s0, 1`.

Marking notes: the bound 8 must be materialised in a register (there is no branch-on-immediate); `i` must be initialised to 0 before the label; the loop must fall through to `Exit` exactly when `i == 8`. The final value of `$s0` is 16 after 8 iterations.
<!-- explanation:end -->
<!-- hint:start -->
The bound cannot be a branch immediate, so it must live in a register; decide where the test sits and where the loop exits.
<!-- hint:end -->

## q25 [hard]

A byte-addressed, little-endian machine executes the following with `$s0 = 0x3000` and `$t0 = 0x00000041`. Give (a) the byte stored at address `0x3002`, and (b) the final value of `$s1`, in hex.

```
sw  $t0, 0($s0)
lui $t1, 0x0000
ori $t1, $t1, 0x1234
srl $t1, $t1, 4
sb  $t1, 2($s0)
lb  $s1, 2($s0)
```

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Step by step:

1. `$t0 = 0x00000041`.
2. `sw $t0, 0($s0)` stores the word `0x00000041` at `0x3000`. Little-endian bytes: `0x3000 = 0x41`, `0x3001 = 0x00`, `0x3002 = 0x00`, `0x3003 = 0x00`.
3. `lui $t1, 0x0000` → `$t1 = 0x00000000`.
4. `ori $t1, $t1, 0x1234` → `$t1 = 0x00001234`.
5. `srl $t1, $t1, 4` → `$t1 = 0x00000123` (logical right shift; zeros fill the top).
6. `sb $t1, 2($s0)` stores the **lower byte** of `$t1`, i.e. `0x23`, at address `0x3002`.

**(a)** The byte at `0x3002` is **`0x23`** (it replaced the `0x00` written by `sw`).

7. `lb $s1, 2($s0)` loads that byte into the **lower** byte of `$s1` and zeroes the upper three bytes.

**(b)** `$s1 = 0x00000023`.

Contrast: `lh`/`sh` (halfword) are out of scope; `lb`/`sb` have no alignment requirement, which is why offset 2 is legal, whereas the `sw` at `0x3000` is legal only because `0x3000` is a multiple of 4.
<!-- explanation:end -->
<!-- hint:start -->
Follow the word's little-endian byte layout, watch the logical shift, and remember `sb` stores only the low byte while `lb` zero-extends.
<!-- hint:end -->

# CS2100 MIPS — Condensed Exam Notes

Distilled from the extended lecture notes (`ch06`–`ch07`). Encode/decode/trace procedures are emphasised. Anything the source does not state is flagged **[not in source]**.

### Design principles (memorise — they explain the "strange facts")

1. Keep the instruction set small. 2. Keep the common part common. 3. Use as much of the part as possible. 4. Keep the instruction size uniform.

### ISA and the translation pipeline

- **ISA**: the *interface* between software and hardware — everything a programmer needs to know to make machine code work correctly. One ISA (e.g. IA-32) has many implementations (Intel 80386 → Pentium 4, AMD, Transmeta) that run *identical* software.
- **Pipeline**: High-level --compile--> Assembly --assemble--> Machine code. Assembly ↔ machine code is usually **one-to-one**, so they are treated as equivalent.
- **Pseudo-instruction**: syntactic sugar, translated by the assembler into one or more **real** instructions. Only **real** instructions count for performance.

| Assembly Language | Machine Code |
|----|----|
| Symbolic version of machine code | Instructions represented in binary |
| Human readable | Technically readable, but hard and tedious |
| `add A, B` | `1000 1100 1010 0000` |

Major instruction classes: **Memory** (move values between memory and registers), **Calculation** (arithmetic/logical), **Control Flow** (change sequential execution).

### Register file (32 registers, each 32 bits = 4 bytes)

Full name/number/usage table is under "Register names → numbers" below. Key facts:

- Convention is only **convention** — nothing stops you using a register otherwise, but you will confuse yourself.
- **No data type** is associated with registers; the value is just binary. The *instruction* assumes the type.
- **`$PC` (Program Counter)**: special register holding the address of the instruction being executed; updated automatically. Decision-making instructions change its next value.
- **[not in source]** `$hi`/`$lo`, `mult`, `div`, `mfhi`, `mflo` are never covered. Only `add`, `sub`, `addi` are treated as arithmetic — multiplication/division are "much more expensive" and deliberately excluded.

### Memory organisation

- Memory is a single-dimension array; each location has an **address**. A *k*-bit address gives 2^k locations.
- **Byte addressing**: each address holds one byte. Instructions, addresses and registers are all 32 bits. Consecutive **words differ by 4**.
- **Word** = usually 2^n bytes, the common transfer unit, same size as register/integer/instruction.
- **Word alignment**: a word is aligned if it begins at a byte address that is a multiple of the word size (multiples of 4 for 4-byte words).
- **Word-align check**: `address mod 4 == 0`, or AND away everything except the last *n* bits and check they are 0.
- MIPS is a **load-store register architecture**: instructions split into memory access and ALU operations; only load/store touch memory.

### Endianness (byte order within a multi-byte word)

| | Big-Endian | Little-Endian |
|----|----|----|
| Order | Most significant byte at the **lowest** address | Least significant byte at the **lowest** address |
| Processors | IBM 360/370, Motorola 6800, **MIPS**, SPARC | Intel 80x86, DEC VAX, DEC Alpha |

`0xDE AD BE EF` is stated to differ between the two, but the actual byte arrangement exists only in an image — **[ambiguous in source]**.

### The 5 general instruction syntaxes

1. **R**: `op $reg, $reg, $reg`  2. **I (immediate)**: `op $reg, $reg, value`  3. **I (branch)**: `op $reg, $reg, label`  4. **I (memory)**: `op $reg, value($reg)`  5. **J**: `op label`

- `value` is typically 16 bits; signed/unsigned depends on the operation.
- **Label**: `label: op ...`. Labels are **NOT** instructions — the assembler removes them, so every I-instruction ends up as `op $reg, $reg, value`.
- Comments start with `#` and run to end of line.

### Instruction formats — exact field widths and order

| Format | Field order (left = most significant) | Widths |
|----|----|----|
| **R** | `opcode` \| `$rs` \| `$rt` \| `$rd` \| `shamt` \| `funct` | 6 \| 5 \| 5 \| 5 \| 5 \| 6 |
| **I** | `opcode` \| `$rs` \| `$rt` \| `immediate` | 6 \| 5 \| 5 \| 16 |
| **J** | `opcode` \| `immediate` | 6 \| 26 |

R fields: `opcode` partially specifies the instruction (**0 for all R-format**); `$rs`/`$rt` = source operands; `$rd` = destination; `shamt` = shift amount (**0 in all non-shift instructions**); `funct` with `opcode` exactly specifies the instruction.
I fields: `opcode` specifies the instruction; `$rs` = first operand; `$rt` = destination **EXCEPT for `sw`, `beq`, `bne`**; `immediate` = constant.

**Trap**: instruction operand order is `$rd | $rs | $rt`, but the *field* order is `$rs | $rt | $rd`. Shifts are written `sll $rd, $rt, shamt` — there is **no `$rs`** in the assembly, so `$rs = 0` in the encoding.

Format usage: branches and load/store are **I-format**; branches use **PC-relative**, load/store use **base/displacement** addressing. Shifts use **R-format**; other immediates (`addi`, `andi`, `ori`, `slti`) use **I-format**. `j` uses **pseudo-direct** addressing.

### Addressing modes

| Mode | Base | Operand comes from | Instructions |
|----|----|----|----|
| **Register** | — | register only | R-format (`add`, `sub`, `sll`, ...) |
| **Immediate** | — | constant inside the instruction | `addi`, `andi`, `ori`, `slti` |
| **Base / displacement** | `$rs` | `MEM[$rs + offset]` | `lw`, `sw`, `lb`, `sb` |
| **PC-relative** | `$PC` | `($PC+4) + (immediate × 4)` | `beq`, `bne` |
| **Pseudo-direct** | upper 4 bits of `$PC+4` | `((PC+4) & 0xF0000000) \| (imm × 4)` | `j` |

### Register names → numbers (with conventional usage)

| No. | Name | Use | No. | Name | Use | No. | Name | Use | No. | Name | Use |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | `$zero` | constant 0 | 8 | `$t0` | temp | 16 | `$s0` | variable | 24 | `$t8` | temp |
| 1 | `$at` | assembler | 9 | `$t1` | temp | 17 | `$s1` | variable | 25 | `$t9` | temp |
| 2 | `$v0` | result | 10 | `$t2` | temp | 18 | `$s2` | variable | 26 | `$k0` | OS |
| 3 | `$v1` | result | 11 | `$t3` | temp | 19 | `$s3` | variable | 27 | `$k1` | OS |
| 4 | `$a0` | argument | 12 | `$t4` | temp | 20 | `$s4` | variable | 28 | `$gp` | global ptr |
| 5 | `$a1` | argument | 13 | `$t5` | temp | 21 | `$s5` | variable | 29 | `$sp` | stack ptr |
| 6 | `$a2` | argument | 14 | `$t6` | temp | 22 | `$s6` | variable | 30 | `$fp` | frame ptr |
| 7 | `$a3` | argument | 15 | `$t7` | temp | 23 | `$s7` | variable | 31 | `$ra` | return addr |

### Opcode / funct values

`funct` (R-format, opcode = 0):

| Op | Hex | Dec | Op | Hex | Dec |
|----|----|----|----|----|----|
| `add` | 20 | 32 | `and` | 24 | 36 |
| `sub` | 22 | 34 | `or` | 25 | 37 |
| `sll` | 00 | 00 | `xor` | 26 | 38 |
| `srl` | 02 | 02 | `nor` | 27 | 39 |

`opcode`:

| Op | Hex | Dec | Op | Hex | Dec |
|----|----|----|----|----|----|
| R-Format | 00 | 00 | `xori` | 0E | 14 |
| `j` | 02 | 02 | `lb` | 20 | 32 |
| `beq` | 04 | 04 | `lw` | 23 | 35 |
| `bne` | 05 | 05 | `sb` | 28 | 40 |
| `addi` | 08 | 08 | `sw` | 2B | 43 |
| `andi` | 0C | 12 | | | |
| `ori` | 0D | 13 | | | |

**[not in source]** `slt`, `slti` and `lui` have no opcode/funct value given anywhere in the source.

### PROCEDURE: encoding an R-format instruction

1. `opcode` = 0. 2. Look up `$rs`. 3. Look up `$rt`. 4. Look up `$rd`. 5. Compute `shamt` (0 if non-shift). 6. Look up `funct`. 7. Convert to binary. 8. Concatenate `opcode|rs|rt|rd|shamt|funct`. 9. Group into 4-bit nibbles → hex.

Worked — `add $t0,$t1,$t2`: rs=9 `01001`, rt=10 `01010`, rd=8 `01000`, shamt=0 `00000`, funct=32 `100000` → `000000 01001 01010 01000 00000 100000` = `0000 0001 0010 1010 0100 0000 0010 0000` = **`0x012A4020`**.
Worked — `sll $t0,$t1,4` (shift: `shamt` used, `$rs = 0`): `000000 00000 01001 01000 00100 000000` = **`0x00094100`**.
Worked — `add $t2,$a3,$a1`: rs=7, rt=5, rd=10, funct=32 → `000000 00111 00101 01010 00000 100000` = **`0x00E55020`**.

### PROCEDURE: encoding an I-format instruction

1. `opcode`. 2. `$rs`. 3. `$rt`. 4. `immediate` (convert to 16-bit 2's complement if negative). 5. Binary. 6. Concatenate `opcode|rs|rt|immediate`.

Assembly-operand ordering differs by class (this affects how you *read the assembly*, not the machine code):

| Instruction | Basic | 1st | 2nd | 3rd |
|----|----|----|----|----|
| **Branch** | `op $rs, $rt, immediate` | `$rs` | `$rt` | `immediate` |
| **Memory** | `op $rt, immediate($rs)` | `$rt` | `immediate` | `$rs` |
| **Others** | `op $rt, $rs, immediate` | `$rt` | `$rs` | `immediate` |

Worked — `addi $s5,$s6,-50`: opcode=8 `001000`, rs=22 `10110`, rt=21 `10101`, imm=-50 = 65536−50 = 65486 = `1111111111001110` → `001000 10110 10101 1111111111001110` = **`0x22D5FFCE`**.
Worked — `lw $t1,12($t0)`: opcode=35 `100011`, rs=8 (base `$t0`), rt=9 (`$t1`), imm=12 → `100011 01000 01001 0000000000001100` = **`0x8D09000C`**. **[Source error]**: the source prints `0x22D5FFCE` here — a copy-paste slip; the binary it prints is correct.
Worked — `ori $t0,$t1,0xFFF` (opcode 13): rs=9, rt=8, imm=4095 → `001101 01001 01000 0000111111111111` = **`0x35280FFF`**. **[Source typo]**: labelled `addi`, but opcode 13 and the tab title show it is `ori`.

### Immediate field: signed vs unsigned

| | Signed `C16_2s` | Unsigned `C16` |
|----|----|----|
| Range | −2^15 .. 2^15−1 (−32768..32767) | 0 .. 2^16−1 (0..65535) |
| Extension to 32 bits | **Sign-extended** (duplicate MSB) | **Bit-extended** (append 0) |
| Used by | `addi`, `slti`, `lw`, `sw` | `andi`, `ori`, `xori` |

There is **no immediate subtraction** and no `nori`: use `addi` with a negative value (design principle #1). `lw`/`sw` offsets are signed 16-bit, i.e. ±32767 bytes from the base.

### PROCEDURE: branch target / branch immediate

- Not taken: `$PC' = $PC + 4`. Taken: `$PC' = ($PC + 4) + (immediate × 4)`.
- `$PC + 4` is the address of the **next** instruction; `$PC'` is the target.
- The immediate counts **instructions to skip over from the next instruction**.
- Diagram method: draw a line *below* the current instruction (= `$PC+4`), a line *above* the target (= `$PC'`), box the region, count instructions inside (= immediate × 4). Target **after** → positive; target **before** → negative.

Worked (forward):

```
Loop: beq  $t1, $zero, End    # rlt addr: 0
      add  $t0, $t0, $t2      # rlt addr: 4
      addi $t1, $t1, -1       # rlt addr: 8
      j    Loop               # rlt addr: 12
End:                          # rlt addr: 16
```

immediate = (16 − (0+4)) / 4 = **3**. Encoding `000100 01001 00000 0000000000000011` = **`0x11200003`**.

Worked (backward): `beq $zero,$zero,Loop` at relative address 12 targeting address 0 → `0 = 16 + imm×4` → **imm = −4**.
Worked (`bne`): `bne $t0,$t1,Loop` at address 12, `Loop` at 0 → imm = −4 = `0xFFFC` = `000101 01000 01001 1111111111111100` = **`0x1509FFFC`**.

Range: the immediate is ±2^15, but because instructions are word-aligned the low 2 bits are always 0, so treating the field as a **word count** (implicit ×4) gives **±2^15 words = ±2^17 bytes**. (One source paragraph phrases this inconsistently as "±2^15 bytes … ±2^13 words" *before* the optimisation; the post-optimisation figure is the encoding's real range.)

### PROCEDURE: encoding `j` (pseudo-direct)

`$PC' = ((PC+4) & 0xF0000000) | (immediate × 4)`, equivalently
`$PC' = ($PC+4) − (($PC+4) % 268435456) + (immediate × 4)` where 268435456 = 2^28.

Reverse procedure: (1) write the target address in binary and note its top 4 bits; (2) write `$PC+4` and note its top 4 bits — they must match or the jump is out of range; (3) drop the top 4 bits and the last 2 bits (always `00`) → the 26-bit immediate.

Worked: target `Loop` = address 8, current `j` at address 20, so `$PC+4` = 24. Target `0000 0000 ... 0000 1000`, `$PC+4` `... 0001 1000`; top 4 bits both `0000`; immediate = 2.
Encoding `000010 00000000000000000000000010` = **`0x08000002`**.

- Max jump range = 2^26 words = 2^28 bytes = **256 MB** boundary.
- Far branches: chain with an intermediate label (`mid_branch: j label`, then `beq ..., mid_branch`).
- Far jumps: chain `j mid_jump` with an always-taken `beq $zero, $zero, label` at/near the 256 MB boundary.

### Instruction summary

| Instruction | Example | Meaning |
|----|----|----|
| `add` | `add $rd, $rs, $rt` | `$rd = $rs + $rt` |
| `sub` | `sub $rd, $rs, $rt` | `$rd = $rs − $rt` (order matters) |
| `addi` | `addi $rt, $rs, imm` | `$rt = $rs + imm` |
| `sll` | `sll $rd, $rt, shamt` | `$rd = $rt << shamt` (fill with 0) |
| `srl` | `srl $rd, $rt, shamt` | `$rd = $rt >> shamt` (fill with 0) |
| `and`/`andi` | `and $rd, $rs, $rt` | bitwise AND (masking / bitmask) |
| `or`/`ori` | `or $rd, $rs, $rt` | bitwise OR (setting / bitset) |
| `xor`/`xori` | `xor $rd, $rs, $rt` | 1 iff bits differ; `a XOR b ≡ a != b` |
| `nor` | `nor $rd, $rs, $rt` | 1 iff both bits are 0; negation of OR |
| `lui` | `lui $rt, imm` | `$rt = imm << 16` |
| `lw` | `lw $rt, offset($rs)` | `$rt = MEM[$rs + offset]` (aligned only) |
| `sw` | `sw $rt, offset($rs)` | `MEM[$rs + offset] = $rt` (aligned only) |
| `lb`/`sb` | `lb $rt, offset($rs)` | byte load/store; **no** alignment requirement |
| `beq` | `beq $rs, $rt, imm` | `$PC = ($rs==$rt) ? ($PC+4)+(imm×4) : ($PC+4)` |
| `bne` | `bne $rs, $rt, imm` | `$PC = ($rs!=$rt) ? ($PC+4)+(imm×4) : ($PC+4)` |
| `slt`/`slti` | `slt $rd, $rs, $rt` | `$rd = ($rs < $rt) ? 1 : 0` |
| `j` | `j imm` | `$PC = (($PC+4) & 0xF0000000) \| (imm×4)` |
| `jr` | `jr $ra` | `$PC = $ra` (switch / procedure return) |
| `jal` | `jal imm` | `$ra = $PC+4`, then as `j` (procedure call) |

| `a` | `b` | `a AND b` | `a OR b` | `a XOR b` | `a NOR b` |
|-----|-----|-----------|----------|-----------|-----------|
| 0 | 0 | 0 | 0 | 0 | 1 |
| 0 | 1 | 0 | 1 | 1 | 0 |
| 1 | 0 | 0 | 1 | 1 | 0 |
| 1 | 1 | 1 | 1 | 0 | 0 |

- No single-operand bitwise NOT exists: use `nor $rd, $rs, $zero`.
- Shift left by *n* = multiply by 2^n; shift right logical by *n* = integer division by 2^n.
- `shamt` is **5-bit unsigned** (0–31) because 2^5 = 32 and shifting a 32-bit word by ≥32 destroys it.

### PROCEDURE: building a 32-bit constant that will not fit in 16 bits

`ori $t0,$zero,0xAAAA` ; `sll $t0,$t0,16` ; `ori $t0,$t0,0xF0F0` gives `0xAAAAF0F0` step by step (`0x0000AAAA` → `0xAAAA0000` → `0xAAAAF0F0`).
Better, because `lui` is a **real** instruction (not a pseudo-instruction) and loads the upper 16 bits with the lower 16 zeroed:
`lui $t0, 0xAAAA` (`0xAAAA0000`) ; `ori $t0, $t0, 0xF0F0` (`0xAAAAF0F0`).
The three-instruction version cannot be merged further because of how the instruction is assembled.

### PROCEDURE: translating C control flow

```
# if (i == j) { f = g + h; }   f=$s0 g=$s1 h=$s2 i=$s3 j=$s4
      bne $s3, $s4, Exit       # inversion: skip body when false
      add $s0, $s1, $s2
Exit:

# if (i == j) { f = g + h; } else { f = g - h; }
      bne $s3, $s4, Else
      add $s0, $s1, $s2
      j   Exit
Else: sub $s0, $s1, $s2
Exit:

# while (j == k) { i = i + 1; }   i=$s3 j=$s4 k=$s5
Loop: bne  $s4, $s5, Exit          # exit when the condition becomes false
      addi $s3, $s3, 1
      j    Loop
Exit:

# for (i=0; i<10; i++) { a = a + 5; }   i=$s0 a=$s2 (10 must live in a register)
      add  $s0, $zero, $zero       # i = 0
      addi $s1, $zero, 10          # $s1 = 10
Loop: beq  $s0, $s1, Exit          # loop ends when i == 10
      addi $s2, $s2, 5             # a = a + 5
      addi $s0, $s0, 1             # i++
      j    Loop
Exit:
```

**Inversion** (invert the condition) is the standard technique for shorter, more uniform code. There is **no real** `blt`/`bgt`/`ble`/`bge`; each is a pseudo-instruction built from `slt` plus one branch:

| Pseudo | Real expansion |
|----|----|
| `blt $rs,$rt,L` | `slt $t0,$rs,$rt` ; `bne $t0,$zero,L` |
| `bgt $rs,$rt,L` | `slt $t0,$rt,$rs` ; `bne $t0,$zero,L` |
| `ble $rs,$rt,L` | `slt $t0,$rt,$rs` ; `beq $t0,$zero,L` |
| `bge $rs,$rt,L` | `slt $t0,$rs,$rt` ; `beq $t0,$zero,L` |

Identities: `$rs > $rt ≡ $rt < $rs`; `$rs <= $rt ≡ !($rt < $rs)`; `$rs >= $rt ≡ !($rs < $rt)`. Also `j label` ≡ `beq $reg, $reg, label` for any valid register.

### PROCEDURE: translating array access

For `int` arrays (4 bytes/element), element *i* sits at offset `4 × i` from the base register (which holds `&A[0]`). `A[7] = h + A[10];` with `h=$s2`, `A=$s3` becomes `lw $t0, 40($s3)` (40 = 4×10) ; `add $t0, $s2, $t0` ; `sw $t0, 28($s3)` (28 = 4×7).

Arithmetic operands must be **registers**, never memory. `lb` loads a byte into the **lower** byte of the destination and zeroes the upper 3 bytes. `ulw`/`usw` (unaligned word load/store) are **pseudo-instructions** that expand into many `lb`/`sb`/`sll`/`srl`/`or` real instructions.

### PROCEDURE: tracing a MIPS snippet

1. Build a state table: every register touched, `$PC`, every touched memory word.
2. Initialise from the stated pre-conditions. **`$zero` is always 0.**
3. Execute one instruction at a time; for R-type, read *all* sources before writing the destination.
4. `beq`/`bne`: decide taken/not taken, then apply the target formula or fall through.
5. An instruction **executes** even when its branch is not taken; a backwards branch taken *k* times means the body ran *k+1* times.
6. Distinguish "branch **executed**" from "branch **taken**": a counter running 10 → 0 checked against 0 makes `bne` execute **11** times and branch **10** times.

Trace A — sum 1..5 with a pseudo-`bge` (`slt` + `beq`):

```
      addi $t0, $zero, 0
      addi $t1, $zero, 1
Loop: slt  $t2, $t1, 6
      beq  $t2, $zero, Exit
      add  $t0, $t0, $t1
      addi $t1, $t1, 1
      j    Loop
Exit:
```

| `$t1` at test | `$t2` = (`$t1`<6) | `$t0` after add | `$t1` after addi |
|----|----|----|----|
| 1 | 1 | 1 | 2 |
| 2 | 1 | 3 | 3 |
| 3 | 1 | 6 | 4 |
| 4 | 1 | 10 | 5 |
| 5 | 1 | 15 | 6 |
| 6 | 0 | branch taken to `Exit` | — |

Final `$t0 = 15`, `$t1 = 6`, `$t2 = 0`. Instruction count = 2 setup + 4 per iteration × 5 iterations + 4 `j` instructions (the `j` is skipped in the final body) = **26**.

Trace B — `sll $t0,$t0,1` / `addi $s1,$s1,-1` with `$t0=2`, `$s1=4`:

| iteration | `$t0` after sll | `$s1` after addi | branch |
|----|----|----|----|
| 1 | 4 | 3 | taken |
| 2 | 8 | 2 | taken |
| 3 | 16 | 1 | taken |
| 4 | 32 | 0 | not taken (exit) |

Final `$t0 = 32` (`0x20`), `$s1 = 0`; body ran 4 times, branch taken 3 times.

Trace C — count non-zero elements of `A[0..4] = {3,0,-1,0,7}`, `$t0 = &A[0] = 0x10010000`:

```
      add  $t6, $zero, $zero
      addi $t9, $t0, 16         # &A[4]
Loop: beq  $t1, $t9, Exit
      lw   $t2, 0($t1)
      beq  $t2, $zero, Skip
      addi $t6, $t6, 1
Skip: addi $t1, $t1, 4
      j    Loop
Exit:
```

| `$t1` | `MEM[$t1]` | `$t6` |
|----|----|----|
| 0x10010000 | 3 | 1 |
| 0x10010004 | 0 | 1 |
| 0x10010008 | -1 | 2 |
| 0x1001000C | 0 | 2 |
| 0x10010010 | 7 | 3 |
| 0x10010014 | — (exits) | 3 |

Final `$t6 = 3`, `$t1 = 0x10010014`.

### ISA beyond MIPS

| | CISC | RISC |
|----|----|----|
| Name | Complex Instruction Set Computer | Reduced Instruction Set Computer |
| Examples | x86-32, IA32 | MIPS, ARM |
| Advantages | Smaller program size (valuable if memory is premium) | Small, simple instruction set; easier to build/optimise hardware |
| Disadvantages | Complex processor implementation; difficult hardware optimisation | Burden on software to compile efficiently |

**5 ISA design concepts**: (1) Data Storage, (2) Memory Addressing Modes, (3) Operations in the Instruction Set, (4) Instruction Formats, (5) Encoding the Instruction Set.

Storage architectures, with `C = A + B`:

| Architecture | Code | Operands | Examples |
|----|----|----|----|
| Stack | `Push A; Push B; Add; Pop C` | implicit, on top of stack | Python bytecode |
| Accumulator | `Load A; Add B; Store C` | one implicit in the accumulator | IBM 701, DEC PDP-8 |
| Register (load-store) | `Load R1,A; Load R2,B; Add R3,R2,R1; Store R3,C` | all explicit | MIPS, DEC Alpha |
| Memory-memory | `Add C,A,B` | all in memory | DEC VAX |

- GPR split into **register-memory** (Motorola 68000, Intel 80386) and **register-register / load-store**. RISC = register-register, CISC mixes the two. MIPS has only **3** addressing modes: register, immediate, displacement.
- **Instruction length**: variable (Intel 80x86 1–17 bytes, VAX 1–54 bytes) needs multi-step fetch/decode but is flexible/compact; fixed (MIPS, PowerPC 4 bytes) is easy to fetch/decode and simplifies pipelining but makes instruction bits scarce; hybrid mixes both.
- **Operand sizes**: character 8 bits, half-word 16, word 32, single-precision float 1 word, double-precision float 2 words.
- **Instruction frequency** (Amdahl — make the common case fast): load 22%, conditional branch 20%, compare 16%, store 12%, add 8%, AND 6%, sub 5%, register move 4%, procedure call 1%, return 1% (96% total).

### Encoding: fixed-length vs expanding opcodes

Fixed-length encoding must fit several instruction types into the same bits; design the **most constrained type first**.

16-bit example — Type-A: 2 × 5-bit operands; Type-B: 1 × 5-bit operand. With a fixed 6-bit opcode (A: `opcode|Operand|Operand`, B: `opcode|Operand|unused`) there are at most 2^6 = 64 instructions and B's 5 bits are wasted. With an **expanding opcode** (B: `opcode|Operand|opcode2`):

- **Maximise total (give Type-A just 1 opcode)**: 1 + (2^6 − 1) × 2^5 = 1 + 2016 = **2017** instructions.
- **Minimise total with the encoding space completely used (give Type-A 2^6−1 opcodes)**: (2^6 − 1) + 2^5 = 63 + 32 = **95**.
- Find the opcode bit budget from the **most restrictive** instruction: `36 = opcode-bits + (2 × 15) + 3` → 3 opcode bits (36-bit instructions, 15-bit addresses, 3-bit registers).
- R-format `funct` is exactly this idea: an extended opcode.

### CS2100 scope reminder

In scope: `add`, `sub`, `addi`; `sll`, `srl`, `and`, `or`, `xor`, `nor`, `andi`, `ori`, `xori`; large constants (`ori`+`sll`+`ori` or `lui`+`ori`); `lw`, `sw`, `lb`, `sb` (plus `ulw`/`usw` as pseudo); `beq`, `bne`, `slt`, `j` and pseudo `blt`/`bgt`/`ble`/`bge`; array access; syscalls (`print_int`, `print_string`, `read_int`, `read_string`, labs only).
Out of scope: procedure support, linkers/loaders/memory layout, stacks/frames/recursion, interrupts/exceptions, `lh`/`sh`/`lwl`/`lwr`/`swl`/`swr`.

### Known source errors and ambiguities (do not memorise these as fact)

1. **Simple-loop Q1** (`ch06b5`): `beq $t1, $zero, Loop` after decrementing `$t1` from 20 never branches, so only 6 instructions run and `$t2` stays 10 — matching the printed answers (a)/(b) but contradicting the lesson's `bne`-to-loop idiom; likely a typo for `bne`.
2. **`lw $t1, 12($t0)`**: binary correct, printed hex (`0x22D5FFCE`) is the previous example's answer. Correct: `0x8D09000C`.
3. **Logical-immediate example** (`ch06c2`) labelled `addi ... 0xFFF`; opcode 13 and the tab title show it should be `ori`.
4. **Summary table** (`ch06b6`) prints the `bne` opcode as `bnq` (typo).
5. **Unaligned store expansion** (`ch06b4`) is internally inconsistent (source register overwritten by `lb`, `$s0` written only at the end, `lb $t0, 4($s1)` repeated for bytes 4/5/6). Learn the concept, not the listing.
6. **Swap example** (`ch06b4`): MipC comments index `$v0` but the code uses `$a0` as the array base and never computes `$v0 = $a0 + k*4`.
7. **`slt`, `slti`, `lui`**: no opcode/funct value anywhere in the source.
8. **Endianness**: the byte layout of `0xDE AD BE EF` exists only as an image; no textual order is given.
9. **Branch range** is phrased inconsistently before/after the implicit ×4 optimisation; use ±2^15 words = ±2^17 bytes.
10. `sra`, `mult`, `div`, `msub`, `mfhi`, `mflo`, `$hi`, `$lo` are **absent from this source**.

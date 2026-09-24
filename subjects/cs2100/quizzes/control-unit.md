# CS2100 Control Unit & ALU Control — Recall Quiz

Scope: the 8 supported single-cycle MIPS instructions only (`add`, `sub`, `and`, `or`, `slt`, `beq`, `lw`, `sw`).
Control-signal order used in every answer: RegDst, ALUSrc, MemToReg, RegWrite, MemRead, MemWrite, Branch, ALUop1, ALUop0, then ALUcontrol.
`X` = don't care. The source's selector implementation emits 0 wherever the truth table says `X`.

## q1 [easy]

The ALU's operation selector `ALUcontrol` is 4 bits wide. Give the 4-bit `ALUcontrol` value that makes the ALU perform subtraction.

<!-- answer:start -->
exact
0110
6
0b0110
<!-- answer:end -->
<!-- explanation:start -->
`ALUcontrol = 0110` is `sub`. Bit 3 = Ainvert = 0, bit 2 = Binvert = 1, bits 1–0 = Operation = 10 (ADD); inverting B turns ADD into A − B. The other values: 0000 `and`, 0001 `or`, 0010 `add`, 0111 `slt`, 1100 `nor`.
<!-- explanation:end -->

## q2 [easy]

`ALUop1` and `ALUop0` are the two bits the main control unit produces from the opcode alone. For an R-format instruction, what are their values? Answer as `ALUop1=?, ALUop0=?`.

<!-- answer:start -->
exact
aluop1=1, aluop0=0
aluop1=1 and aluop0=0
aluop1 = 1, aluop0 = 0
aluop1 = 1 and aluop0 = 0
10
1 0
<!-- answer:end -->
<!-- explanation:start -->
`ALUop` table: `lw`/`sw` → ALUop1=0, ALUop0=0; `beq` → ALUop1=0, ALUop0=1; R-format → ALUop1=1, ALUop0=0. R-format cannot be resolved from the opcode alone, so `ALUop1=1` marks "look at funct".
<!-- explanation:end -->

## q3 [easy]

For which of the eight supported instructions does the control unit produce `ALUop1 = 0, ALUop0 = 1`?

<!-- answer:start -->
exact
beq
branch on equal
beq (branch on equal)
<!-- answer:end -->
<!-- explanation:start -->
`ALUop1=0, ALUop0=1` is the `beq` encoding. `beq` needs subtraction so that `isZero?` can test `$rs == $rt`. (`lw`/`sw` are ALUop1=0, ALUop0=0; R-format is ALUop1=1, ALUop0=0.)
<!-- explanation:end -->

## q4 [easy]

Inside one 1-bit ALU slice, what does the control bit `Binvert = 1` do to the input `B`?

<!-- answer:start -->
exact
invert input b
invert b
inverts input b
inverts b
it inverts input b
invert the b input
bitwise not of input b
<!-- answer:end -->
<!-- explanation:start -->
`Binvert` (1 bit): 0 = do not invert input `B`; 1 = invert input `B`. It is one of the four 1-bit-ALU controls (`Ainvert`, `Binvert`, `Operation1`, `Operation0`), and it is exactly what makes `sub` (ALUcontrol 0110) possible from an adder.
<!-- explanation:end -->

## q5 [easy]

What value does `MemWrite` take while `lw $8, 4($17)` executes? Give 0 or 1.

<!-- answer:start -->
exact
0
zero
false
<!-- answer:end -->
<!-- explanation:start -->
For `lw`: MemRead=1, MemWrite=0. `MemRead` and `MemWrite` must never both be 1; a load only reads. (`MemWrite=1` is `sw` only.)
<!-- explanation:end -->

## q6 [easy]

Which two control signals must never both be 1 at the same time?

<!-- answer:start -->
exact
memread and memwrite
memread, memwrite
memread & memwrite
memread/memwrite
memread memwrite
memread and memwrite signals
<!-- answer:end -->
<!-- explanation:start -->
`MemRead` and `MemWrite` can never both be 1, so only three combinations are legal: `00`, `01`, `10`. `01` is `sw`, `10` is `lw`, `00` is every other instruction.
<!-- explanation:end -->

## q7 [easy]

Which supported instruction sets `Branch = 1`?

<!-- answer:start -->
exact
beq
branch on equal
beq (branch on equal)
<!-- answer:end -->
<!-- explanation:start -->
`Branch` is the selector for `beq` only. It then combines with the ALU's `isZero?` output: `PCSrc = Branch & isZero?`. For all other instructions `Branch = 0`, so `$PC' = $PC+4`.
<!-- explanation:end -->

## q8 [medium]

Give the **complete** control-signal vector for `lw $8, 4($17)` (opcode 0x23 = 100011). State every one of: RegDst, ALUSrc, MemToReg, RegWrite, MemRead, MemWrite, Branch, ALUop1, ALUop0, ALUcontrol.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Expected vector: **RegDst=0, ALUSrc=1, MemToReg=1, RegWrite=1, MemRead=1, MemWrite=0, Branch=0, ALUop1=0, ALUop0=0, ALUcontrol=0010.**

Reasoning: opcode 100011 ⇒ `lw` (I-format), so a 9-bit control word from the truth table row `lw` = `0 1 1 1 1 0 0 0 0`. ALUop1=0/ALUop0=0 forces ALUcontrol=0010 (ADD) because the lw/sw "add base+offset" case ignores funct. RegDst=0 selects `$rt` = `Inst[20:16]` as the write register; ALUSrc=1 selects `sign_extend(Inst[15:0])` as the ALU's second operand (the address comes from Read Data 1 + immediate, not from Read Data 2); MemRead=1 with MemWrite=0; MemToReg=1 because the flipped mux must pass memory `Read Data` back into the register file; RegWrite=1; Branch=0.
<!-- explanation:end -->

## q9 [medium]

Give the **complete** control-signal vector for `sw $8, 0($17)` (opcode 0x2B = 101011). State every one of: RegDst, ALUSrc, MemToReg, RegWrite, MemRead, MemWrite, Branch, ALUop1, ALUop0, ALUcontrol.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Expected vector from the truth table: **RegDst=X, ALUSrc=1, MemToReg=X, RegWrite=0, MemRead=0, MemWrite=1, Branch=0, ALUop1=0, ALUop0=0, ALUcontrol=0010.** The source's selector implementation actually emits RegDst=0 and MemToReg=0 for `sw` (neither is R-format, neither is `lw`); both are acceptable because they are don't-cares.

Reasoning: `sw` writes no register (RegWrite=0), so RegDst and MemToReg cannot affect anything. ALUSrc=1 is required so the ALU adds the sign-extended offset to Read Data 1 and produces the store address. MemWrite=1, MemRead=0: the data written is `Read Data 2` (i.e. `$rt`). ALUop1=0/ALUop0=0 ⇒ ALUcontrol=0010 (ADD). Branch=0.
<!-- explanation:end -->

## q10 [medium]

Give the **complete** control-signal vector for `beq $16, $0, Else` (opcode 0x04 = 000100). State every one of: RegDst, ALUSrc, MemToReg, RegWrite, MemRead, MemWrite, Branch, ALUop1, ALUop0, ALUcontrol. Then state the value of PCSrc if `$16 == $0`.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Expected vector from the truth table: **RegDst=X, ALUSrc=0, MemToReg=X, RegWrite=0, MemRead=0, MemWrite=0, Branch=1, ALUop1=0, ALUop0=1, ALUcontrol=0110.** The source's selector implementation emits RegDst=0 and MemToReg=0 (don't-cares, since RegWrite=0).

Reasoning: `beq` performs `$rs − $rt` (ALUcontrol 0110, from ALUop1=0/ALUop0=1) and tests `isZero?`. ALUSrc=0 keeps `Read Data 2` (= `$rt`) as the second ALU operand — the immediate is NOT fed to the ALU; it goes to the separate branch-target adder `($PC+4) + (immediate×4)`. No memory access, no register write. With Branch=1 and isZero=1, `PCSrc = Branch & isZero? = 1`, so `$PC' = ($PC+4) + (immediate×4)`.
<!-- explanation:end -->

## q11 [medium]

Give the **complete** control-signal vector for `sub $8, $8, $16` (opcode 000000, funct 100010). State every one of: RegDst, ALUSrc, MemToReg, RegWrite, MemRead, MemWrite, Branch, ALUop1, ALUop0, ALUcontrol.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Expected vector: **RegDst=1, ALUSrc=0, MemToReg=0, RegWrite=1, MemRead=0, MemWrite=0, Branch=0, ALUop1=1, ALUop0=0, ALUcontrol=0110.**

Reasoning: opcode 000000 ⇒ R-format, so the base row is `1 0 0 1 0 0 0 1 0` (RegDst=1, ALUSrc=0, MemToReg=0, RegWrite=1, MemRead=0, MemWrite=0, Branch=0, ALUop1=1, ALUop0=0). Because ALUop1=1, funct decides ALUcontrol: funct 100010 has F1=1 and F2=0, so ALUcontrol2=(F1·ALUop1)+ALUop0=1, ALUcontrol1=!ALUop1+!F2=1, ALUcontrol0=(F0+F3)·ALUop1=0, ALUcontrol3=0 ⇒ 0110 = `sub`. RegDst=1 selects `$rd`=`Inst[15:11]` as the destination; ALUSrc=0 means the second operand is `Read Data 2` (`$rt`).
<!-- explanation:end -->

## q12 [medium]

A control unit emits this signal pattern for one instruction. Which of the eight supported instructions is it?

| RegDst | ALUSrc | MemToReg | RegWrite | MemRead | MemWrite | Branch | ALUop1 | ALUop0 | ALUcontrol |
|---|---|---|---|---|---|---|---|---|---|
| X | 1 | X | 0 | 0 | 1 | 0 | 0 | 0 | 0010 |

<!-- answer:start -->
exact
sw
store word
sw (store word)
<!-- answer:end -->
<!-- explanation:start -->
It is `sw`. Discriminators: `MemWrite=1` and `MemRead=0` occur only for `sw`; `ALUSrc=1` with `ALUop1=0, ALUop0=0` is the lw/sw address-add case (ALUcontrol 0010 = ADD); `RegWrite=0` rules out R-format and `lw`; `Branch=0` rules out `beq`. The two `X`s are RegDst and MemToReg, which are irrelevant because nothing is written back (the source's selector logic emits RegDst=0, MemToReg=0).
<!-- explanation:end -->

## q13 [medium]

A control unit emits this signal pattern for one instruction. Which of the eight supported instructions is it?

| RegDst | ALUSrc | MemToReg | RegWrite | MemRead | MemWrite | Branch | ALUop1 | ALUop0 | ALUcontrol |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0111 |

<!-- answer:start -->
exact
slt
set less than
set on less than
<!-- answer:end -->
<!-- explanation:start -->
It is `slt`. `RegWrite=1` with no memory access and `ALUop1=1` means R-format; `RegDst=1` selects `$rd`; `ALUSrc=0` uses `Read Data 2`; and ALUcontrol=0111 is the unique R-format operation code for "set on less than" (its funct is 101010, giving F3=1 ⇒ ALUcontrol0=1 and F1=1 ⇒ ALUcontrol2=1). `add` would be 0010, `sub` 0110, `and` 0000, `or` 0001.
<!-- explanation:end -->

## q14 [medium]

An R-format instruction has `funct = 100101`. Using the simplified truth table, what 4-bit `ALUcontrol` does the ALU control unit produce?

<!-- answer:start -->
exact
0001
1
0b0001
<!-- answer:end -->
<!-- explanation:start -->
`funct = 100101` is `or`. From the simplified truth table (ALUop1=1 ⇒ R-format): F2=1 so ALUcontrol1 = !ALUop1 + !F2 = 0; F1=0 so ALUcontrol2 = (F1·ALUop1) + ALUop0 = 0; F0=1 so ALUcontrol0 = (F0+F3)·ALUop1 = 1; ALUcontrol3 = 0. Result = 0001 (`or`).
<!-- explanation:end -->

## q15 [medium]

For `beq`, give the value of `ALUcontrol` and explain exactly how the branch decision is produced from the ALU output and the `Branch` signal.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`ALUcontrol = 0110` (`sub`), because `ALUop1=0, ALUop0=1` and funct is a don't care. The ALU computes `Read Data 1 − Read Data 2` = `$rs − $rt`, and equality is tested as `A == B ⇔ A − B == 0`, so the ALU's 1-bit `isZero?` output is 1 exactly when `$rs == $rt`. That single bit is combined with the control-unit output: **`PCSrc = Branch & isZero?`**. With Branch=1 (set only for `beq`), PCSrc=1 only when the subtraction produced zero, selecting `$PC' = ($PC+4) + (immediate×4)`; otherwise the mux keeps `$PC+4`. Note there is no dedicated "equality" ALU operation.
<!-- explanation:end -->

## q16 [medium]

The `nor` operation is supported by the 1-bit ALU slice. Give the values of `Ainvert`, `Binvert` and `Operation` it uses, and give its 4-bit `ALUcontrol` code.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`nor`: **Ainvert=1, Binvert=1, Operation=00 (AND), ALUcontrol=1100.** Both inputs are inverted and then ANDed, and `~(A | B) = ~A & ~B` by De Morgan, which is exactly NOR. Bit layout: ALUcontrol = Ainvert · Binvert · Operation1 · Operation0 = 1 1 00 = 1100.
<!-- explanation:end -->

## q17 [medium]

A `beq` instruction is executing and the ALU's `isZero?` output is 1. What is `PCSrc`, and what is the next value of `$PC`?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`PCSrc = Branch & isZero? = 1 & 1 = 1`, so the taken branch address is selected: **`$PC' = ($PC+4) + (immediate×4)`**. The `immediate×4` term is the sign-extended 16-bit branch offset shifted left by 2 (word alignment), and `$PC+4` is the already-computed next-sequential address. If `isZero?` had been 0, `PCSrc=0` and `$PC' = $PC+4`.
<!-- explanation:end -->

## q18 [medium]

For `sw`, the truth table marks `RegDst` and `MemToReg` as `X`. Explain why each is a don't care, and state what the source's selector-based implementation actually outputs for them.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`sw` has `RegWrite = 0`, so the register file is never written: which register number the RegDst mux selects, and which value the MemToReg mux forwards as `Write Data`, can have no effect. They are don't cares in the truth table. In the selector implementation, `RegDst` is 1 only for R-format and `MemToReg` is 1 only for `lw`; `sw` is neither, so the hardware actually emits **RegDst = 0 and MemToReg = 0**. Either value satisfies the truth table. (`beq` has the same two don't cares for the same reason.)
<!-- explanation:end -->

## q19 [hard]

Derive the Boolean expression the source uses for `ALUcontrol2`, justifying each term from the simplified truth table. The equations for the other bits are `ALUcontrol3=0`, `ALUcontrol1=!ALUop1+!F2`, `ALUcontrol0=(F0+F3)·ALUop1`.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`ALUcontrol2 = (F1 · ALUop1) + ALUop0`.

Justification: scanning the `ALUcontrol2` column of the simplified truth table, the 1s are `beq` (0110), `sub` (0110) and `slt` (0111). `sub` has F1=1 and `slt` has F1=1, and both are R-format, so "R-format AND F1" gives `F1 · ALUop1`. The remaining 1 is `beq`, which is uniquely identified by `ALUop0` (it is the only instruction with ALUop0=1), giving the term `ALUop0`. All other rows are 0: `lw`/`sw` (0010) and `add` (0010) have F1=0 and ALUop0=0; `and` (0000) and `or` (0001) have F1=0. Since either condition may hold, OR the two terms. Note `ALUop1` must gate the F1 term: F1 alone would wrongly make `lw`/`sw` produce 0110.
<!-- explanation:end -->

## q20 [hard]

The source's 1-bit ALU documents `Operation` values `00` (AND), `01` (OR) and `10` (ADD) only. Which `Operation` value must `slt` use, and what does the source say about the implementation of that path? State any part you cannot verify from the source.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
`slt` uses **Operation = 11** together with **Ainvert = 0, Binvert = 1**, giving `ALUcontrol = 0111`. The source explicitly says: "the implementation for `slt` is not shown", and it never defines what `Operation=11` does internally — so the set/less-than path (the extra logic that produces a 0/1 result, and the least-significant-slice set output) cannot be verified from `control.md`. Likewise, although the source shows that the 1-bit full adder has a `Cin` input and that slices chain `Cout → Cin`, it never states the carry-in trick for subtraction. [NOT IN SOURCE — outside knowledge]: subtraction is normally done by asserting `Binvert=1` and forcing `Cin=1` into bit 0 so that `A + ~B + 1 = A − B`; `slt` is normally done by computing `A − B` and routing the sign bit of the result (with overflow correction) to the least-significant bit. Do not present those as sourced facts.
<!-- explanation:end -->

## q21 [hard]

A `sw` instruction (opcode 101011) reaches the control unit, but the assembler has left `funct = 100010` in the low 6 bits. What 4-bit `ALUcontrol` is generated, and why is `funct` irrelevant here? Verify your answer with the Boolean equations.

<!-- answer:start -->
exact
0010
2
0b0010
<!-- answer:end -->
<!-- explanation:start -->
`ALUcontrol = 0010` (ADD). `sw` is opcode 101011, so the main control unit sets ALUop1=0, ALUop0=0; the ALU control unit only consults `funct` when ALUop1=1 (R-format), so all other instructions get ADD regardless of the trapped bits. Check: ALUcontrol3 = 0; ALUcontrol2 = (F1·ALUop1)+ALUop0 = (1·0)+0 = 0; ALUcontrol1 = !ALUop1 + !F2 = 1+1 = 1; ALUcontrol0 = (F0+F3)·ALUop1 = (0+0)·0 = 0 ⇒ 0010.
<!-- explanation:end -->

## q22 [hard]

A student builds the datapath but ties `ALUSrc` permanently to 0. Which supported instructions still work correctly, and which break? Explain the failure mode.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Still correct: the five R-format instructions and `beq` — all of them have `ALUSrc=0` in the truth table, so they are unaffected. They take their second ALU operand from `Read Data 2` (`$rt`).

Broken: `lw` and `sw`, both of which need `ALUSrc=1`. With `ALUSrc` stuck at 0 the ALU computes `Read Data 1 + Read Data 2` = `$rs + $rt` instead of `$rs + sign_extend(immediate)`, so the memory address is wrong and the load/store hits an unrelated location (the offset field is never sign-extended or added). Fix: `ALUSrc` must be the `lw OR sw` selector.
<!-- explanation:end -->

## q23 [hard]

A student sets `MemToReg = 0` for every instruction, including `lw`. What value ends up in the destination register, and why? What is the correct value and what does it select?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The destination register receives the **ALU result**, i.e. the computed memory address `$rs + sign_extend(offset)` — not the loaded word. The source notes that the `MemToReg` multiplexer is *inverted*: `0` selects the ALU result and `1` selects memory `Read Data`. The correct value for `lw` is therefore **MemToReg = 1**, which routes the data read from memory into `Write Data`. For R-format, MemToReg = 0 is correct (ALU result). `sw`/`beq` have RegWrite=0 so their MemToReg value cannot matter.
<!-- explanation:end -->

## q24 [hard]

The single-cycle design must fit every instruction into one clock period. Assuming Memory = 2 ns, ALU/Adder = 2 ns and Register = 1 ns, what clock period in nanoseconds must the design use? Give the number only.

<!-- answer:start -->
numeric 0
8
<!-- answer:end -->
<!-- explanation:start -->
8 ns. The per-instruction totals given by the source are: R-format 2+1+2+1 = 6 ns; `lw` 2+1+2+2+1 = 8 ns; `sw` 2+1+2+2 = 7 ns; `beq` 2+1+2 = 5 ns. A single-cycle design has one clock period for all instructions, so it must accommodate the slowest — `lw` at 8 ns. This is the core limitation that motivates multicycle execution (one smaller clock cycle per stage, variable cycle count per instruction) and pipelining (overlapping stages of successive instructions so idle components such as Instruction Memory are used).
<!-- explanation:end -->

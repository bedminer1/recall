# CS2100 — Single-Cycle MIPS Control (Condensed Exam Notes)

Source: `control.md` (ch08, ch08a1–a6, ch08b, ch08c1–c4). Every table below is copied from the source.
This file is also the answer key for `control-unit.md`. Anything the source does not state is marked **[NOT IN SOURCE]**.

### 0. Scope: what this datapath actually supports

The source builds the *simplest possible* subset of MIPS:

| Class | Instructions supported |
|---|---|
| Arithmetic / logical (R-format) | `add`, `sub`, `and`, `or`, `slt` |
| Data transfer (I-format) | `lw`, `sw` |
| Branch (I-format) | `beq` |

* `sll` / `srl` (shift) and the J-format `j` are **explicitly NOT implemented** — "left as exercise for the reader".
* `andi` / `ori` are struck out: "these two instructions cannot be fully implemented correctly using our current datapath and/or control implementation".
* ch08 line 23 also lists `bne`, but the supported-instruction list in ch08c1 and the control truth table contain **only `beq`**. Treat `beq` as the only branch.
* **There is NO `Jump` control signal in this design** (no J-format hardware exists). If an exam asks for "Jump", the source has no answer for it — say so.

### 1. The five stages

1. **Fetch** — read instruction from `Instruction Memory` at `$PC`; compute `$PC+4` with an adder.
2. **Decode & Operand Fetch** — read `opcode`; read up to 2 registers (`$rs` → Read Register 1, `$rt` → Read Register 2). Decode and operand fetch are merged because MIPS is fixed-length.
3. **ALU** — arithmetic/logic, address calculation, branch comparison, branch-target adder.
4. **Memory** — `Data Memory` read or write (never both).
5. **Writeback** — route the correct result back into the `Register File`.

Design decision (source): merge decode + operand fetch; split the old "execute" into ALU and Memory.

### 2. Control signals — full definitions (8 identifier signals)

| Signal | Stage | Purpose | Values |
|---|---|---|---|
| `RegDst` | Decode | Select the destination register number | `0`: `$rt` = `Inst[20:16]`; `1`: `$rd` = `Inst[15:11]` |
| `RegWrite` | Decode/Writeback | Enable writing of register | `0`: no write; `1`: write |
| `ALUSrc` | ALU | Select the 2nd operand for ALU | `0`: `Read Data 2` (`$rt`); `1`: `sign_extend(Inst[15:0])` |
| `ALUcontrol` | ALU | Select the ALU operation | see `ALUcontrol` table (4 bits) |
| `MemRead` | Memory | Enable reading of data memory | `0`: no read; `1`: read at ALU `result` |
| `MemWrite` | Memory | Enable writing of data memory | `0`: no write; `1`: write at ALU `result`, data = `Read Data 2` |
| `MemToReg` | Writeback | Select the value written back | `0`: ALU result; `1`: memory data |
| `Branch` | Memory/Writeback | Select the next `$PC` value | `0`: `$PC+4`; `1`: `($PC+4)+(immediate×4)` |

* `MemRead` and `MemWrite` must **never both be 1** → only `00`, `01`, `10` are legal.
* The `MemToReg` multiplexer is **inverted/flipped**: `1` = top input = memory `Read Data`, `0` = `ALU result`.
* `Branch` is *sometimes written* as `PCSrc`, but `PCSrc` is normally `Branch & isZero?`. The source uses both names loosely — see §18.
* `ALUcontrol` is the 4-bit *operation* selector; `ALUop` is an intermediate 2-bit signal (see §6). `ALUop` is not listed among the "8 identifier control signals" because it is internal to the control/ALU-control split.

### 3. `ALUcontrol` operation encoding (convention, not a law)

| `ALUcontrol` | Function |
|---|---|
| 0000 | `and` |
| 0001 | `or` |
| 0010 | `add` |
| 0110 | `sub` |
| 0111 | `slt` |
| 1100 | `nor` |

Equality is **not** an ALU operation: `$rs == $rt` is computed as `$rs - $rt == 0` and read off the `isZero?` output.

### 4. The 1-bit ALU (bit slice)

4 control bits are needed:

* `Ainvert` (1 bit) — `0`: do not invert `A`; `1`: invert `A`.
* `Binvert` (1 bit) — `0`: do not invert `B`; `1`: invert `B`.
* `Operation` (2 bits) — `00`: bitwise AND; `01`: bitwise OR; `10`: ADD. (`11` is not documented — see §18.)

Mapping from `ALUcontrol` to bit-slice controls:

| Function | Ainvert | Binvert | Operation | ALUcontrol |
|---|---|---|---|---|
| `and` | 0 | 0 | 00 | 0000 |
| `or` | 0 | 0 | 01 | 0001 |
| `add` | 0 | 0 | 10 | 0010 |
| `sub` | 0 | 1 | 10 | 0110 |
| `slt` | 0 | 1 | 11 | 0111 |
| `nor` | 1 | 1 | 00 | 1100 |

Bit order: `ALUcontrol = Ainvert · Binvert · Operation1 · Operation0` (bit 3 = Ainvert, bit 2 = Binvert, bits 1–0 = Operation).

So: **`sub` = add with `B` inverted** (`Binvert=1`, `Operation=10`); **`nor` = invert both then AND**; **`slt` needs `Binvert=1` and `Operation=11`**.
The source states: "the implementation for `slt` is not shown". **[NOT IN SOURCE]**: the carry-in-to-LSB trick (set `Cin=1` into bit 0 so `A + ~B + 1 = A - B`) and the set/less-than path are not described in `control.md`; the source only shows that a full adder has a `Cin` input and that slices chain `Cout → Cin`.

### 5. The 32-bit ALU

A 32-bit ALU is built by chaining 32 copies of the abstracted 1-bit slice: the `Cout` of slice *i* feeds the `Cin` of slice *i+1*. All slices share `Ainvert`, `Binvert`, `Operation` (i.e. one shared `ALUcontrol`). The ALU outputs `A op B` (32 bits) and `(A op B) == 0?` (`isZero`, 1 bit).

### 6. Multilevel decoding — the `ALUop` intermediate signal

Instead of brute-forcing a 12-input function (`opcode` + `funct`), the source first derives an intermediate signal that **depends only on `opcode`**:

| Instruction Type | `ALUop` |
|---|---|
| `lw` / `sw` | 00 |
| `beq` | 01 |
| R-Format | 10 |

The two bits are `ALUop1` (MSB) and `ALUop0` (LSB); where the source writes the pair as `10`, `01`, `00` it means `ALUop1 ALUop0`. In answers write `ALUop1=…, ALUop0=…`.

Rationale: `lw`/`sw` need ADD (base + offset); `beq` needs SUB (equality test); R-format needs "something else", so the decision is deferred to `funct`.

Then `ALUcontrol` = f(`ALUop1`/`ALUop0`, 6-bit `funct`). This reduces the size of the main controller and can speed up the circuit.

### 7. ALU control — summary table

| Opcode | ALUop | Instruction | Funct | Action | ALUcontrol |
|---|---|---|---|---|---|
| `lw` | 00 | Load | `XX XXXX` | Add | 0010 |
| `sw` | 00 | Store | `XX XXXX` | Add | 0010 |
| `beq` | 01 | Branch | `XX XXXX` | Subtract | 0110 |
| R-Format | 10 | Add | `10 0000` | Add | 0010 |
| R-Format | 10 | Subtract | `10 0010` | Subtract | 0110 |
| R-Format | 10 | AND | `10 0100` | AND | 0000 |
| R-Format | 10 | OR | `10 0101` | OR | 0001 |
| R-Format | 10 | Less Than | `10 1010` | Less Than | 0111 |

Conventions: `1` = true, `0` = false, `X` = don't care (output: value is irrelevant; input: must work for either value), `-` = cannot matter (input that cannot affect the output). Operators used: `+` OR, `.` AND, `!` NOT.

### 8. ALU control — full truth table

`F0`–`F5` are bits 0–5 of `funct`; `ALUop0`, `ALUop1` are bits 0, 1; `ALUcontrol0`–`ALUcontrol3` are bits 0–3.
Funct is written MSB-first below (`F5 F4 F3 F2 F1 F0`).

| | ALUop | Funct | ALUcontrol |
|---|---|---|---|
| `lw` | `0 0` | `X X X X X X` | `0 0 1 0` |
| `sw` | `0 0` | `X X X X X X` | `0 0 1 0` |
| `beq` | `0 1` | `X X X X X X` | `0 1 1 0` |
| `add` | `1 0` | `1 0 0 0 0 0` | `0 0 1 0` |
| `sub` | `1 0` | `1 0 0 0 1 0` | `0 1 1 0` |
| `and` | `1 0` | `1 0 0 1 0 0` | `0 0 0 0` |
| `or` | `1 0` | `1 0 0 1 0 1` | `0 0 0 1` |
| `slt` | `1 0` | `1 0 1 0 1 0` | `0 1 1 1` |

### 9. ALU control — simplified truth table

Obtained by deleting inputs that cannot differentiate any output: `F5`, `F4` are always `10` for R-format; for R-format `ALUop1=1` and `ALUop0` is irrelevant (`1 -`); for `beq` `ALUop0=1` and `ALUop1` is irrelevant (`- 1`).

| | ALUop | Funct | ALUcontrol |
|---|---|---|---|
| `lw` | `0 0` | `X X X X X X` | `0 0 1 0` |
| `sw` | `0 0` | `X X X X X X` | `0 0 1 0` |
| `beq` | `- 1` | `X X X X X X` | `0 1 1 0` |
| `add` | `1 -` | `- - 0 0 0 0` | `0 0 1 0` |
| `sub` | `1 -` | `- - 0 0 1 0` | `0 1 1 0` |
| `and` | `1 -` | `- - 0 1 0 0` | `0 0 0 0` |
| `or` | `1 -` | `- - 0 1 0 1` | `0 0 0 1` |
| `slt` | `1 -` | `- - 1 0 1 0` | `0 1 1 1` |

### 10. ALU control unit — Boolean equations (the implementation)

```
ALUcontrol3 = 0
ALUcontrol2 = (F1 . ALUop1) + (ALUop0)
ALUcontrol1 = !ALUop1 + !F2
ALUcontrol0 = (F0 + F3) . ALUop1
```

How each equation is justified (source's reasoning pattern — you are expected to reproduce it):

* `ALUcontrol3`: always 0 for every supported instruction.
* `ALUcontrol2`: 1 when `F1=1` **and** R-format (`ALUop1=1`), OR when it is `beq` (`ALUop0=1`).
* `ALUcontrol1`: 0 only when `ALUop1=1` **and** `F2=1`; negate that condition → `!(ALUop1 . F2)` = De Morgan → `!ALUop1 + !F2`. For all non-R-format it is simply 1.
* `ALUcontrol0`: 1 for `or` (`F0=1`) or `slt` (`F3=1`), but only for R-format, hence `(F0 + F3) . ALUop1`.

### 11. Main control unit — opcode map

`Op5`–`Op0` are bits 5–0 of the 6-bit `opcode`.

| | Op5 | Op4 | Op3 | Op2 | Op1 | Op0 | Hexadecimal |
|---|---|---|---|---|---|---|---|
| R-Format | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `lw` | 1 | 0 | 0 | 0 | 1 | 1 | 23 |
| `sw` | 1 | 0 | 1 | 0 | 1 | 1 | 2B |
| `beq` | 0 | 0 | 0 | 1 | 0 | 0 | 4 |

`opcode = Inst[26:31]`; `funct = Inst[0:5]`. Only the opcode enters the main control unit; only `opcode` (→`ALUop1`/`ALUop0`) and `funct` enter the ALU control unit.

### 12. Main control unit — truth table

`Ctrl` is the merged output vector. Bit positions: `Ctrl0`=RegDst, `Ctrl1`=ALUSrc, `Ctrl2`=MemToReg, `Ctrl3`=RegWrite, `Ctrl4`=MemRead, `Ctrl5`=MemWrite, `Ctrl6`=Branch, `Ctrl7`=ALUop1, `Ctrl8`=ALUop0.

| | RegDst | ALUSrc | MemToReg | RegWrite | MemRead | MemWrite | Branch | ALUop1 | ALUop0 |
|---|---|---|---|---|---|---|---|---|---|
| R-Format | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 |
| `lw` | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 0 | 0 |
| `sw` | X | 1 | X | 0 | 0 | 1 | 0 | 0 | 0 |
| `beq` | X | 0 | X | 0 | 0 | 0 | 1 | 0 | 1 |

Merged form: R-Format `1 0 0 1 0 0 0 1 0`; `lw` `0 1 1 1 1 0 0 0 0`; `sw` `X 1 X 0 0 1 0 0 0`; `beq` `X 0 X 0 0 0 1 0 1`.

### 13. Main control unit — selector-based implementation

Each `Ctrl` bit is a **selector**: it is 1 exactly when the current instruction is one of the instructions in its list. Build the selector for an opcode by NOT-ing the opcode bits that are 0 and AND-ing all six (with bubbles): R-format = all six opcode bits 0; `lw` = `100011`; `sw` = `101011`; `beq` = `000100`.

| Ctrl | Signal | Selector |
|---|---|---|
| 0 | `RegDst` | R-Format |
| 1 | `ALUSrc` | `lw` OR `sw` |
| 2 | `MemToReg` | `lw` |
| 3 | `RegWrite` | R-Format OR `lw` |
| 4 | `MemRead` | `lw` |
| 5 | `MemWrite` | `sw` |
| 6 | `Branch` | `beq` |
| 7 | `ALUop1` | R-Format |
| 8 | `ALUop0` | `beq` |

Selectors are combined with OR gates only. Because `sw`/`beq` have `X` for `RegDst`/`MemToReg`, this implementation emits a concrete value for them: **`RegDst=0` and `MemToReg=0`** for both `sw` and `beq` (neither is R-Format, and neither is `lw`). Those values are legal but arbitrary — the truth table marks them don't-care.

### 14. Per-instruction control vectors (memorise these)

| Instruction | RegDst | ALUSrc | MemToReg | RegWrite | MemRead | MemWrite | Branch | ALUop1 ALUop0 | ALUcontrol |
|---|---|---|---|---|---|---|---|---|---|
| `add $rd,$rs,$rt` | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 10 | 0010 |
| `sub $rd,$rs,$rt` | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 10 | 0110 |
| `and $rd,$rs,$rt` | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 10 | 0000 |
| `or $rd,$rs,$rt` | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 10 | 0001 |
| `slt $rd,$rs,$rt` | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 10 | 0111 |
| `lw $rt,off($rs)` | 0 | 1 | 1 | 1 | 1 | 0 | 0 | 00 | 0010 |
| `sw $rt,off($rs)` | X (0) | 1 | X (0) | 0 | 0 | 1 | 0 | 00 | 0010 |
| `beq $rs,$rt,label` | X (0) | 0 | X (0) | 0 | 0 | 0 | 1 | 01 | 0110 |

For `beq`, the final next-PC decision is `PCSrc = Branch & isZero?` = `1 & isZero?`. If `$rs == $rt` (isZero=1) then `$PC' = ($PC+4) + (immediate×4)`, otherwise `$PC' = $PC+4`.

### 15. Datapath routing, stage by stage

* **Fetch** — `$PC` → `Instruction Address` of `Instruction Memory`; the same `$PC` goes to an adder (`+4`); instruction output goes to decode. `$PC` is read in the first half of the cycle and updated at the next rising edge.
* **Decode** — `opcode = Inst[26:31]` goes to the control unit; `$rs = Inst[25:21]` → Read Register 1 → `Read Data 1`; `$rt = Inst[20:16]` → Read Register 2 → `Read Data 2`. Write-register mux: `RegDst=0` → `Inst[20:16]` (`$rt`, used by `lw`), `RegDst=1` → `Inst[15:11]` (`$rd`, used by R-format). `Inst[15:0]` goes through `sign_extend`.
* **ALU** — operand 1 is always `Read Data 1`; operand 2 mux: `ALUSrc=0` → `Read Data 2`, `ALUSrc=1` → `sign_extend(Inst[15:0])`. The ALU produces `result` and `isZero`. In parallel, a separate adder computes the branch target `($PC+4) + (immediate × 4)` — this is *not* the ALU, so `beq` still has `ALUSrc=0`.
* **Memory** — address is the ALU `result`. `MemRead=1` → data read out; `MemWrite=1` → write `Read Data 2` into memory. `sw`/`beq`: nothing happens.
* **Writeback** — `MemToReg` mux (flipped) picks ALU `result` (0) or memory `Read Data` (1) into `Write Data`; the register number comes from the decode stage mux; the actual write happens only if `RegWrite=1`. `$PC` is updated here in the walkthrough: `PCSrc` selects `$PC+4` or the target.

### 16. PROCEDURE — answering "give every control signal for this instruction"

1. **Decode the format.** Opcode `000000` → R-format (fields `rs`,`rt`,`rd`,`shamt`,`funct`). Opcode `100011` → `lw`, `101011` → `sw`, `000100` → `beq` (I-format: `rs`,`rt`,`immediate`). Anything else is outside the supported subset.
2. **Write out the two ALUop bits.** `lw`/`sw` → `ALUop1=0, ALUop0=0`; `beq` → `ALUop1=0, ALUop0=1`; R-format → `ALUop1=1, ALUop0=0`.
3. **If R-format, read `funct` (`Inst[0:5]`)** and cross-reference the simplified truth table (or apply the four Boolean equations) to get the 4-bit `ALUcontrol`. If not R-format, `funct` is a don't care — `ALUcontrol` comes from `ALUop1`/`ALUop0` alone.
4. **Set `RegDst`.** R-format → 1 (`$rd`). `lw` → 0 (`$rt`). `sw`/`beq` → X (implementation gives 0). Remember: `RegDst` matters only when `RegWrite=1`.
5. **Set `ALUSrc`.** `lw`/`sw` → 1 (needs the sign-extended immediate for the address). R-format/`beq` → 0.
6. **Set `RegWrite`.** R-format/`lw` → 1. `sw`/`beq` → 0.
7. **Set the memory pair.** `lw` → `MemRead=1, MemWrite=0`. `sw` → `MemRead=0, MemWrite=1`. R-format/`beq` → `0,0`. Never both 1.
8. **Set `MemToReg`.** `lw` → 1 (memory data). R-format → 0 (ALU result). `sw`/`beq` → X (implementation gives 0). Remember the mux is flipped.
9. **Set `Branch`.** `beq` → 1, everything else → 0.
10. **Finish with the derived signals:** `ALUcontrol` (step 3) and `PCSrc = Branch & isZero?` (only 1 for `beq` when `$rs == $rt`).
11. **Sanity checks.** Exactly one of `MemRead`/`MemWrite` may be 1; `sw` and `beq` never write a register; `ALUop1=1` (R-format) must be accompanied by `RegDst=1` and `ALUSrc=0`.

Reverse direction ("which instruction is this pattern?"): use the discriminating signals first — `Branch=1` ⇒ `beq`; `MemWrite=1` ⇒ `sw`; `MemRead=1` (and `RegWrite=1`) ⇒ `lw`; `ALUop1=1` ⇒ R-format, then `ALUcontrol` picks which R-format instruction.

### 17. Big picture

* Any instruction must read storage (register/memory), compute through combinational logic, then write storage — so a clock edge must separate the read from the write.
* **Single cycle** does all of that in one clock period; the period must cover the *slowest* instruction.

| Instruction | Inst. Mem | Reg. Read | ALU | Data Mem | Reg. Write | Total |
|---|---|---|---|---|---|---|
| R-Format | 2 | 1 | 2 | | 1 | 6 ns |
| `lw` | 2 | 1 | 2 | 2 | 1 | 8 ns |
| `sw` | 2 | 1 | 2 | 2 | | 7 ns |
| `beq` | 2 | 1 | 2 | | | 5 ns |

Given Memory 2 ns, ALU/Adder 2 ns, Register 1 ns, **every** instruction takes 8 ns because `lw` is the slowest.
* **Multicycle** breaks the instruction into execution steps (e.g. one per stage); each step is 1 (smaller, faster) clock cycle, and an instruction may take a variable number of cycles.
* **Pipelining** goes further: while one instruction is in the ALU, `Instruction Memory` is idle, so fetch/decode the next instruction. Idle components get used; new hazards appear (covered next semester).
* Why single-cycle control is simple: one instruction per cycle means every control signal is a pure combinational function of the instruction's opcode (and, for `ALUcontrol`, `funct`) — no state, no sequencing, no per-cycle signal changes.

### 18. Ambiguities and contradictions found in the source

1. **`Jump` does not exist here.** The task description mentions `Jump`, but J-format `j` is explicitly not implemented and no `Jump` signal appears anywhere in `control.md`. There is no examinable Jump value in this source.
2. **`Branch` vs `PCSrc`.** ch08a3 calls the branch-taken control signal `PCSrc`; ch08c1 calls it `Branch` and its footnote says "`PCSrc` is often refer to `Branch & isZero?` instead". The truth table uses `Branch`. This note uses `Branch` for the control-unit output and `PCSrc` for the derived mux select.
3. **`bne` vs `beq`.** ch08's intro lists `beq` *and* `bne`, but the supported-instruction list and control truth table contain only `beq`. If asked about `bne`, the source gives no control vector.
4. **Decode-stage signal names/values (ch08a2) are corrupted.** The numbered list gives `RegDst` twice (item 3 is clearly meant to be `ALUSrc`), and item 3's value for the immediate path says `Inst[15:11]` — it should be `sign_extend(Inst[15:0])` as stated in ch08c1. Item 1 (`Inst[20:16]`→0, `Inst[15:11]`→1) is correct and matches ch08c1.
5. **`slt` and `Operation=11` are undocumented.** The 1-bit ALU lists only `Operation` 00/01/10, yet the `slt` row needs `11`, and the source says "the implementation for `slt` is not shown". The subtraction carry-in trick (LSB `Cin=1`) is likewise never spelled out — only the abstract `Cin` of the full adder and `Cout → Cin` chaining are shown.
6. **`ALUcontrol1` has two forms.** The source first derives `(ALUop1 . !F2) + (!ALUop1)` and then, via De Morgan, presents `!ALUop1 + !F2`. Both are equivalent; the final boxed form is the second.
7. **`ALUcontrol0` is asserted without derivation** — the source presents `(F0 + F3) . ALUop1` and asks the reader to justify it. It does check out against the simplified truth table (`or` → `F0=1`, `slt` → `F3=1`, both restricted to R-format).
8. **`sw` labelled R-Format in ch08b.** In the assembler walkthrough, `sw` is introduced as "*R-Format*" while the fields shown (`opcode` 43 = `101011`, `$rs`, `$rt`, `immediate`) are clearly **I-format**. The field table is right; the label is a typo.
9. **Instruction-format list typo.** In ch08c1's instruction table, `beq` is written as `beq $rd, $rs, label`; `beq` takes `$rs` and `$rt` (as shown everywhere else in the source).
10. **`addi` appears in a decode example but is not in the supported subset.** ch08a2 traces `addi $21, $22, -50` to motivate the `RegDst`/`ALUSrc` muxes; ch08c1's supported list and truth table do not include `addi`. There is no control vector for `addi` in the truth table (and `addi` would need `RegDst=0`, `ALUSrc=1`, `RegWrite=1`, with `ALUop1`/`ALUop0` never stated — so do not quote a vector for it).

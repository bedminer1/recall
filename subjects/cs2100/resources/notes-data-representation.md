# CS2100 Data Representation & Number Systems — Condensed Exam Notes

Source: extended lecture notes `ch03`–`ch03f2`. Ambiguities/typos in the source are flagged **[SOURCE ISSUE]**.

### Scope
Convert whole numbers between any bases; represent negatives in sign-and-magnitude, 1s/2s complement and excess-N; represent reals in fixed-point and IEEE 754 floating-point.

### Weighted-positional definition
A base/radix *R* system is **weighted-positional**: a symbol's value depends on its position. Position 0 is the first digit *left* of the dot; left **increments** the position, right **decrements** it (negative positions allowed).

> Value = Σ cᵢ × Rⁱ (general base b; also quoted as Σ_{i=0}^{∞} cᵢ bⁱ)

(7594.36)₁₀ = (7×10³)+(5×10²)+(9×10¹)+(4×10⁰)+(3×10⁻¹)+(6×10⁻²) = 7594.36.

Conventions: subscript = base; no subscript ⇒ base 10 **unless clear from context**. Symbol count normally equals the base; symbols ordered so each is one more than the previous, and 0 is always 0. Base = integer ≥ 2.

| Radix | Name | Symbols |
|----|----|----|
| 2 | Binary | bits: 0, 1 |
| 8 | Octal | 0, 1, 2, 3, 4, 5, 6, 7 |
| 16 | Hexadecimal | 0–9, A, B, C, D, E, F |
| R | base/radix R | digits ≥ 10 use `A`–`Z`, so bases up to 36 (26 letters + 10 digits) |

Prefixes: C octal `0` (`032`), hex `0x` (`0x32`), binary `0b` (`0b10100`); MIPS simulator uses `0x` for hex; Verilog `8'b11110000` = `8'hF0` = `8'd240`. Corner cases (not examined): base 1 = unary (represented by string length); non-integer base, e.g. (10001010)√2 = 23, and base √2 → base 2 deletes the (zero) odd-position bits.

### Powers of 2 to memorise
| 2⁺ | Val | 2⁻ | Val | 2⁺ | Val | 2⁻ | Val |
|----|----|----|----|----|----|----|----|
| 2¹ | 2 | 2⁻¹ | 0.5 | 2⁶ | 64 | 2⁻⁶ | 0.015625 |
| 2² | 4 | 2⁻² | 0.25 | 2⁷ | 128 | 2⁻⁷ | 0.0078125 |
| 2³ | 8 | 2⁻³ | 0.125 | 2⁸ | 256 | 2⁻⁸ | 0.00390625 |
| 2⁴ | 16 | 2⁻⁴ | 0.0625 | 2⁹ | 512 | 2⁻⁹ | 0.001953125 |
| 2⁵ | 32 | 2⁻⁵ | 0.03125 | 2¹⁰ | 1024 | 2⁻¹⁰ | 0.0009765625 |

### PROCEDURE — base R → base 10
1. Split at the dot into `<whole number>` and `<fraction>` (both integers).
2. Weight each digit by R^(position), position 0 at the dot, +1 per step left, −1 per step right.
3. Multiply and sum. Skip digits that are 0 (0 × Rⁿ = 0 → shortcut).
Worked: (1101.101)₂ = 8+4+0+1+0.5+0+0.125 = **13.625**; (572.6)₈ = 320+56+2+0.75 = **378.75**; (2A.8)₁₆ = 32+10+0.5 = **42.5**.
Quiz answers: (1101001.0110)₂ = 64+32+8+1+0.25+0.125 = **105.375**; (6204.12)₈ = 3072+128+4+0.125+0.03125 = **3204.15625** **[SOURCE ISSUE: the printed working says (6224.12)₈, a typo]**; (CA.FE)₁₆ = 192+10+0.9375+0.0546875 = **202.9921875**.

### PROCEDURE — base 10 → base R
| Kind | Method | Why |
|----|----|----|
| Whole number | Repeated **division** by *R* | weight is multiplied by R going 10 → R, so reverse is division |
| Fraction | Repeated **multiplication** by *R* | weight is divided by R going 10 → R, so reverse is multiplication |

**Repeated division (whole part):** integer-divide by R, *prepend* each remainder, stop at quotient 0, read bottom-up (last remainder = MSB).
```
res := ""
while (num > 0) do
  res := (num % R) ++ res   # prepend remainder
  num := num / R            # integer division
end
```
43 → base 2:

| R | num | Division | Remainder | |
|----|----|----|----|----|
| 2 | 43 | 43/2 = 21 | 43%2 = 1 | ← LSB |
| 2 | 21 | 21/2 = 10 | 21%2 = 1 | |
| 2 | 10 | 10/2 = 5 | 10%2 = 0 | |
| 2 | 5 | 5/2 = 2 | 5%2 = 1 | |
| 2 | 2 | 2/2 = 1 | 2%2 = 0 | |
| 2 | 1 | 1/2 = 0 | 1%2 = 1 | ← MSB |
| 2 | 0 | | | **STOP** |

⇒ (101011)₂. Continuing past 0 only appends value-less leading zeros, so stopping at 0 is a choice.

**Repeated multiplication (fraction part):** multiply the fraction by R, *append* the truncated whole part, keep only the fractional part each step.
```
res := ""
while (num > 0) do
  num := num * R            # repeated multiplication
  res := res ++ trunc(num)  # truncate whole part, append
end
```
0.3125 → base 2:

| R | num | Multiplication | Truncation | |
|----|----|----|----|----|
| 2 | 0.3125 | ×2 = 0.625 | ⌊0.625⌋ = 0 | ← MSB |
| 2 | 0.625 | ×2 = 1.25 | ⌊1.25⌋ = 1 | |
| 2 | 0.25 | ×2 = 0.5 | ⌊0.5⌋ = 0 | |
| 2 | 0.5 | ×2 = 1.0 | ⌊1.0⌋ = 1 | ← LSB |
| 2 | 0 | | | **STOP** |

⇒ (0.0101)₂. Termination is **not guaranteed**: (0.1)₁₀ cycles 0.2 ⇝ 0.4 ⇝ 0.8 ⇝ 0.6 ⇝ 0.2 (digits 0,0,0,1,1,…), and because the algorithm is deterministic it never terminates. Practical rule: **limit the fraction to at most 10 digits**. 0.999… (9 repeated) = 1.
Quiz answers: 2100 → base 3 = (**2212210**)₃ (remainders 0,1,2,2,1,2,2 bottom-up); 0.111…₁₀ → base 3 = (**0.01**)₃ (×3 = 0.333… → 0; ×3 = 0.999… → 1, using 0.999… = 1).
Base-2100 exercises: smallest valid base 3 (digit 2 present); largest base keeping value ≤ 1000 is 7, since (2100)₇ = 735 but (2100)₈ = 1088; smallest base giving ≥ 3000 is 12, since (2100)₁₁ = 2783 but (2100)₁₂ = 3600.

### PROCEDURE — base R1 → base R2
General route: R1 → base 10 (weighted-positional) → R2 (repeated division/multiplication).

| From | To | Shortcut |
|----|----|----|
| 2 | 8 | group **3** bits from the dot, pad leading/trailing zeros, convert each group |
| 8 | 2 | each octal digit → exactly 3 bits; drop leading/trailing zeros |
| 2 | 16 | group **4** bits from the dot, pad, convert each group |
| 16 | 2 | each hex digit → exactly 4 bits; drop leading/trailing zeros |
| R | Rⁿ | **"other cases" trick**: if the target is a power of the source, use groups of *n* digits |
| both powers of 2 | | go through binary and apply grouping twice |

Grouping is safe because 3 bits max = 7 and 4 bits max = 15 = F.
Worked: (10111011001.101110)₂ = (2731.56)₈ = (5D9.B8)₁₆; (2731.56)₈ = (10111011001.10111)₂; (10110010101.0100101)₂ = (2625.224)₈; (21.02)₈ = (10001.00001)₂; (110100100101001.01010001011)₂ = (6929.516)₁₆; (BABA.9090)₁₆ = (1011101010111010.100100001001)₂.
Exercise answers: (12202.012)₃ → base 9 (3² = 9, groups of 2): 01 22 02 . 01 20 ⇒ **182.16**; (CA.FE)₁₆ → base 4 (16 = 4², groups of 2): C A . F E ⇒ 30 22 . 33 32 ⇒ **3022.3332**; (7345)₈ → base 16: 111 011 100 101 → regroup 1110 1110 0101 ⇒ **EE5**.

### ASCII and character encoding
Character representation has no natural ordering — it is pure **convention**. Standard: **ASCII** (American Standard Code for Information Interchange), 7 bits plus one **parity bit** (ignored here; a leading 0 is appended unless stated). Parity: *odd* ⇒ total number of 1s including the parity bit is odd; *even* ⇒ total is even. E.g. `1001101` + odd parity → `10011011`; `0110101` + even parity → `00110101`. Order: **digits < uppercase < lowercase**. Anchors: `'0'` = 48, `'A'` = 65, `'a'` = 97. Examples: `A` = 1000001 (65), `r` = 1110010 (114), `F` = 1000110 (70). In C chars and ints 0–127 are interchangeable (`%c`/`%d` reinterpret the same byte). Extended Unicode's first 128 code points are ASCII; it adds all other scripts and emoji (not examined).

### Binary addition (unsigned warm-up)
Adding 1: any digit below the max becomes the next digit; at 9 (base 10) or 1 (binary), write 0 and carry 1 left. Long addition = add the two digits **plus the carry**. The binary result may need one extra bit, then gets **truncated** to the fixed width. Subtraction is x − y ≡ x + (−y), so only negation and addition must be implemented.

### Sign-and-magnitude (sm)
Sign bit before the magnitude: **0 = positive, 1 = negative**; so the number of bits must be fixed (otherwise you cannot tell which bit is the sign: `1100101` vs `1000000100101` both claim −100101). Negation: **flip the left-most bit only**, bidirectionally, with no need to know the decimal value. Problems (8-bit): redundant bit for small numbers, limited range, **redundant 0** (+0 = `00000000`, −0 = `10000000`). Benefit: simple, mirrors math notation, single-bit negation; addition is not practical.

| Kind | Representation | Decimal |
|----|----|----|
| Largest | 0111…11 | +(2ⁿ⁻¹ − 1) |
| Smallest | 1111…11 | −(2ⁿ⁻¹ − 1) |

8-bit: +127 (`01111111`) to −127 (`11111111`) = 255 values, one lost to redundant 0 (unsigned 8 bits has 256). Derivation with n−1 ones at positions n−2…0: S = 2ⁿ⁻²+…+2⁰, 2S − S = 2ⁿ⁻¹ − 2⁰ = 2ⁿ⁻¹ − 1. Total values = **2ⁿ − 1**.

### 1s complement
Negation: **flip all bits**. Formula: **−x = 2ⁿ − x − 1** (n = bits, x = value negated). Proof sketch: 2ⁿ = 1 followed by n zeros; 2ⁿ−1 = n ones; (2ⁿ−1) − x is an XOR of x with all ones = bit flip.
Worked: 8-bit −14 ⇒ 14 = 00001110 ⇒ flip ⇒ **(11110001)₁s** (formula: 2⁸−14−1 = 241 = 11110001); 8-bit −80 ⇒ **(10101111)₁s**. 
| Kind | Representation | Decimal |
|----|----|----|
| Largest | 0111…11 | +(2ⁿ⁻¹ − 1) |
| Smallest | 1000…00 | −(2ⁿ⁻¹ − 1) |
| Positive 0 | 0000…00 | +0 |
| Negative 0 | 1111…11 | −0 |

(−2ⁿ⁻¹) is not representable: it would be the flip of (+2ⁿ⁻¹) = 1000…00, giving 0111…11, which is positive. Range is again only **2ⁿ − 1** values; redundant 0 remains.

| Value | Binary | Value | Binary |
|----|----|----|----|
| 0 | 0000 | −7 | 1000 |
| 1 | 0001 | −6 | 1001 |
| 2 | 0010 | −5 | 1010 |
| 3 | 0011 | −4 | 1011 |
| 4 | 0100 | −3 | 1100 |
| 5 | 0101 | −2 | 1101 |
| 6 | 0110 | −1 | 1110 |
| 7 | 0111 | −0 | 1111 |

x + (−x) = x + (2ⁿ−x−1) = 2ⁿ − 1, which is the −0 codeword — the "zero" of this system. **[SOURCE ISSUE: the source writes this chain but concludes "= 0"; the honest intermediate value is 2ⁿ−1, the negative-zero codeword]**

Addition needs an **end-around carry**:
```
C := A + B                            # plain binary addition
if num_bits(C) == num_bits(A) + 1 do  # carried out / crossed the redundant 0
  C := C + 1                          # add the carry back in
end
check_overflow(A,B,C)
C := truncate(C)                      # keep n bits
```
**Overflow rule (1s and 2s):** overflow can only occur when **both operands have the same sign**, and it has occurred iff the **result's sign differs** from the operands'. Different-sign operands can never overflow.
Worked 4-bit: 3+4 = 0111 = +7, no overflow; 5+(−5) = 0 101 + 1 010 = 1 111 = −0, no overflow; (−2)+(−5) = 1 101 + 1 010 = 10 111 → end-around ⇒ 1 000 = −7, no overflow; (−3)+(−7) = 1 100 + 1 000 = 10 100 → +1 ⇒ 0 101, **overflow** (negatives gave a positive). 7+7 = 0111+0111 = 1110 = −1 ⇒ overflow.
Quizzes (4-bit): 4−7 = 0100+1000 = 1100 = **−3, no overflow** (different signs); 3−(−5) = 0011+0101 = 1000, **overflow** (same signs, sign flipped); 7−0 = 0111+1111 = 10110 → end-around ⇒ 0111 = **+7, no overflow**.

### 2s complement
Removes redundant 0 by making 1111…11 represent **−1** (shift every negative by one). Formula: **−x = 2ⁿ − x**. Negation: **flip all bits, then add 1**; the magnitude goes up by one relative to 1s complement.
Worked: 8-bit −12 ⇒ 12 = 00001100 ⇒ invert 11110011 ⇒ +1 ⇒ **(11110100)₂s** (formula: 2⁸−12 = 244); 8-bit −80 ⇒ **(10110000)₂s**.

| Kind | Representation | Decimal |
|----|----|----|
| Largest | 0111…11 | +(2ⁿ⁻¹ − 1) |
| Smallest | 1000…00 | −2ⁿ⁻¹ |
| 0 | 0000…00 | 0 |

All 2ⁿ codewords are used; range is asymmetric (one more negative than positive).
Addition: plain binary addition, truncate, then the same overflow rule (no end-around step).
```
C := A + B            # binary addition
check_overflow(A,B,C)
C := truncate(C)
```
Worked 4-bit: 3+4 = 0111 = +7 (no overflow); 5+(−5) = 1 011+0 101 = (1)0 000 → 0000 = 0 (no overflow, despite the carry out); (−2)+(−6) = 11 000 → 1000 = −8 (no overflow); 6+(−3) = 10 011 → 0011 = +3 (no overflow); 4+(−7) = 1101 = −3 (no overflow); (−3)+(−6) = 10 111 → 0111 = +7 **overflow**; 5+6 = **1011 = −5 overflow**.
Quizzes (4-bit): −1−7 = 1111+1001 = 11 000 → 1000 = **−8, no overflow** (operands negative, result stays negative); 3−(−5) = 0011+0101 = 1000, **overflow** (should be +8); 7−0 = 0111+0000 = **+7, no overflow**.

### Excess-N (biased)
Purpose: remove the discontinuity when incrementing (1111 → 0000 must not be a value jump), which makes comparison hard. Method: don't start counting at 0; let 000…00 represent the **starting number −N**. "Excess-N on M-bit numbers" ⇒ start = −N, M bits. Codeword value = (unsigned value of bits) − N.

| Excess-4 | Value | Excess-4 | Value |
|----|----|----|----|
| 000 | −4 | 100 | 0 |
| 001 | −3 | 101 | 1 |
| 010 | −2 | 110 | 2 |
| 011 | −1 | 111 | 3 |

Excess-2 on 3-bit: 000=−2, 001=−1, 010=0, 011=1, 100=2, 101=3, 110=4, 111=5. Excess-7 on 3-bit: 000=−7 … 011=−4, 100=−3 … 111=0. Excess-7 on 4-bit: 0000 = −7 … 0111 = 0 … 1111 = 8.

| Excess-8 | Value | Excess-8 | Value |
|----|----|----|----|
| 0000 | −8 | 1000 | 0 |
| 0001 | −7 | 1001 | 1 |
| 0010 | −6 | 1010 | 2 |
| 0011 | −5 | 1011 | 3 |
| 0100 | −4 | 1100 | 4 |
| 0101 | −3 | 1101 | 5 |
| 0110 | −2 | 1110 | 6 |
| 0111 | −1 | 1111 | 7 | **[SOURCE ISSUE: the source's Excess-8 table prints the second column as 0…7 (copied from Excess-7), contradicting the "unsigned − 8" rule; use the corrected table above. The 3-bit Excess-4/Excess-7 tables and the 8-bit exercise are self-consistent.]**
Range: to distribute evenly, shift 0…2ⁿ−1 so the start is −N with **N = 2ⁿ⁻¹** (−2ⁿ⁻¹ … 2ⁿ⁻¹−1). The source also allows **N = 2ⁿ⁻¹ − 1** (favours positives). **IEEE uses Excess-(2ⁿ⁻¹ − 1)**, e.g. 127 for 8 bits.
Even-distribution answers: 8-bit ⇒ Excess-128 or Excess-127 (2⁷); 11-bit ⇒ Excess-1024 or Excess-1023; 16-bit ⇒ Excess-32768 or Excess-32767.
Arithmetic: **not** plain binary addition — adding −4 and −3 in Excess-4 on 3-bit gives 000+001 = 001 = −3, clearly wrong. The advantage is **comparison only**: because it is a pure translation, comparing two Excess-N numbers is a plain binary comparison.

### Summary: four representations on 3-bit numbers

| Value | Sign-and-Magnitude | 1s Complement | 2s Complement | Excess-4 | Excess-3 |
|----|----|----|----|----|----|
| −4 | — | — | 100 | 000 | |
| −3 | 111 | 100 | 101 | 001 | 000 |
| −2 | 110 | 101 | 110 | 010 | 001 |
| −1 | 101 | 110 | 111 | 011 | 010 |
| −0 | 100 | 111 | | | |
| +0 | 000 | 000 | 000 | 100 | 011 |
| +1 | 001 | 001 | 001 | 101 | 100 |
| +2 | 010 | 010 | 010 | 110 | 101 |
| +3 | 011 | 011 | 011 | 111 | 110 |
| +4 | | | | | 111 |

Discontinuity when incrementing past the top (4-bit): sm +7 → −0; 1s +7 → −7; 2s +7 → −8.

### Summary: operations (8-bit example, value 23)

| Operation | Sign-and-Magnitude | 1s Complement | 2s Complement | Excess-N |
|----|----|----|----|----|
| Negation | invert left-most bit | invert all bits | invert all bits, add 1 | — |
| Addition | — | binary addition, add carry, truncate | binary addition, truncate | — |
| Comparison | — | — | — | plain binary comparison |

- sm: 23 = 00010111 → flip first bit ⇒ 10010111 = −23 → flip back ⇒ 23.
- 1s: 00010111 → invert ⇒ 11101000 = −23 → invert ⇒ 00010111 = 23.
- 2s: 00010111 → invert 11101000 → +1 ⇒ 11101001 = −23 → invert 00010110 → +1 ⇒ 00010111 = 23.

Comparison: same-sign positives → plain binary compare; different signs → sign bit 0 is larger. Two negatives only: **sm** larger = *smaller* tail bits (tail = magnitude); **1s and 2s** larger = *larger* tail bits. Excess-N removes this entirely.

### Radix-complement formula (any base b, n digits)

| Digits | Radix | (b−1)s complement (diminished radix) | (b)s complement (radix) |
|----|----|----|----|
| n | b | −x = bⁿ − x − 1 | −x = bⁿ − x |

Example: (43)₁₀ as 5 trits = (01121)₃. (b−1)s: 3⁵−43−1 = 199 = (21101)₃. (b)s: 3⁵−43 = 200 = (21102)₃.
10s-complement quizzes: −2100 in 5 digits = 10⁵−2100 = **(97900)₁₀ₛ**; 2100−1010: −1010 ⇒ 10⁵−1010 = 98990; 2100+98990 = 101090; truncate to 5 digits ⇒ **(01090)₁₀ₛ** ("borrowing taken to the extreme": 10s-complement addition is plain decimal addition + truncation).

### Fractions variant of the complement formulas
Remove the dot, complement, put the dot back. With n whole bits and f fraction bits, the smallest representable magnitude is **2⁻ᶠ**.

| Whole bits | Fraction bits | 1s complement | 2s complement |
|----|----|----|----|
| n | f | −x = 2ⁿ − x − 2⁻ᶠ ; invert all bits | −x = 2ⁿ − x ; invert all bits **and add 2⁻ᶠ** |

Worked (negate 5.25, 4 whole + 2 fraction bits): 5.25 = 0101.01 ⇒ invert ⇒ **(1010.10)₁s** (formula 2⁴−5.25−2⁻² = 16−5.25−0.25 = 10.5 = 1010.10); then add 2⁻² = 0.01 ⇒ **(1010.11)₂s** (formula 2⁴−5.25 = 10.75 = 1010.11 — the 2s formula is unchanged; only the "add 1" shortcut becomes "add 2⁻ᶠ").
Quiz answers: negate (111000.101) with 6 whole + 3 fraction bits ⇒ 1s **(000111.010)₁s**, 2s **(000111.011)₂s**.

### Fixed-point representation
The fractions extension of 1s/2s complement: fix how many bits are whole and how many are fraction, so the **binary point sits at a fixed position**. The dot is purely imaginary — no bit stores it. Works with any negative representation (Excess-N rarely used).
Examples with 6 whole + 2 fraction bits: (26.75)₁₀ = (011010.11); (−1.25)₁₀ = (111110.10)₁s and (111110.11)₂s (from 1.25 = 000001.01, invert, then +2⁻² for 2s).
**Resolution** = smallest representable amount = **2⁻ᶠ**. All representable numbers are integer multiples of the resolution, and the multiple's ordinary complement bit pattern with the point re-inserted is exactly the fixed-point pattern. With f = 2 (resolution 0.25): 26.75 = 107 × 0.25 (107 = 01101011) and −1.25 = −5 × 0.25 (−5 = 11111010 in 1s, 11111011 in 2s).
**Approximation**: numbers that are not multiples of the resolution cannot be represented. (0.125)₁₀ = (0.001)₂ needs 3 fraction bits, so with 2 you must round up to (000000.01)₂ = 0.25 or down to (000000.00)₂ = 0. Usual policy: **truncate** to the available bits (other rounding schemes out of scope).
Exercise: −36.03125 as 16-bit fixed-point 1s complement, 10 whole + 6 fraction bits: 36.03125 = 100100.00001₂ → pad to 10.6 = 0000100100.000010 → invert ⇒ **1111011011.111101₁s**.

### Floating-point representation (IEEE 754)
Scientific notation in binary, `<sign> <mantissa> × 2^<exponent>`; the base 2 is implicit and never stored. This gives far more range than fixed-point because the point floats.

| Format | Total bits | Sign | Mantissa (normalised) | Exponent |
|----|----|----|----|----|
| `float` single precision | 32 | 1 | 23 | 8, **Excess-127** |
| `double` double precision | 64 | 1 | 52 | 11, **Excess-1023** |

Memorise only total bits and exponent bits; infer the rest: sign is always 1 bit; mantissa = total − exponent − 1; excess = 2^(exponent bits − 1) − 1 because IEEE prefers favouring positive exponents (8 bits ⇒ 2⁷−1 = 127, not 128).
**Normalised mantissa / hidden bit**: binary has only two symbols, so a normalised mantissa is always `1.xxxx`; the leading 1 is assumed and **not stored**, giving one free bit of precision. This trick works only in binary (decimal has too many possible leading digits).
**Why excess for the exponent**: floating-point add/subtract requires comparing exponents (the result takes the larger), and Excess-N makes that a plain binary comparison.
Worked (−6.5)₁₀ as `float`: −6.5 = −110.1₂ = −1.101₂ × 2²; sign = 1; exponent = 2 + 127 = 129 = `10000001`; mantissa = `101` padded to 23 bits ⇒ (sign 1 | exponent 10000001 | mantissa 10100000000000000000000)₂ = **C0D00000₁₆**.
Worked exercise −36.03125₁₀ ⇒ **0xC2102000**: −100100.00001₂ = −1.0010000001₂ × 2⁵; sign = 1; exponent = 5 + 127 = 132 = `10000100`; mantissa = `0010000001` padded to 23 bits ⇒ (1 10000100 00100000010000000000000)₂ = 1100 0010 0001 0000 0010 0000 0000 0000 = C2102000₁₆.

### Past-year exam question (silent overflow in C)
`int i, n = 2147483640; for (i = 1; i <= 10; i++) { n = n + 1; } printf("n = %d\n", n);`
Output on sunfire: **`n = -2147483546`**. C uses 2s complement and `int` is 32 bits, so max = 2³¹−1 = 2147483647 and min = −2³¹ = −2147483648. Trace: 2147483640, …641, …642, …643, …644, …645, …646, **…647 (max)**, then iteration 8 wraps to **−2147483648 (min)**, then −2147483647, then −2147483646.

### Exercise to memorise (8-bit, −55)
55₁₀ = 00110111₂ ⇒ sm **10110111**; 1s **11001000**; 2s **11001001**; Excess-128 (00000000 = −128): 128 − 55 = 73 = **01001001**.

### [SOURCE ISSUE] list
1. **ASCII digit list typo**: source prints "0, , 1, 2, …, 9" (stray comma).
2. **(6204.12)₈ printed as (6224.12)₈** in the quiz working; the evaluated terms (3072+128+4+…) correspond to 6204.
3. **1s complement "x + (−x) = 0"** is derived as 2ⁿ−1 and then labelled 0; 2ⁿ−1 is the −0 codeword, not the all-zero codeword.
4. **Excess-8 4-bit table is wrong** (second column printed 0…7, should be 8…15); it duplicates Excess-7.
5. **Even-parity definition typo**: both bullet definitions say "odd"; the even rule is that the total count of 1s is even (the worked example is consistent with the even rule).
6. **0.111… → base 3**: source says "look at the first 3 digits" but performs only 2 multiplications; result (0.01)₃ is correct because 0.111… = 1/9 = 3⁻².
7. **10s-complement truncation**: the source never states the modulus explicitly; the rule is truncate to n digits (101090 → 01090 with n = 5).
8. **Excess offset not unique**: the source allows N = 2ⁿ⁻¹ and N = 2ⁿ⁻¹−1 and calls the off-by-one "usually okay"; IEEE mandates 2ⁿ⁻¹−1. Only answer uniquely when the format is named.

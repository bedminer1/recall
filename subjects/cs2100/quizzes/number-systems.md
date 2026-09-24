# CS2100 Number Systems & Data Representation — Computational Recall Quiz

Answer all questions showing working where asked. Difficulty tags: `[easy]` one idea directly, `[medium]` combines steps, `[hard]` transfer / debugging / multi-step comparison.

## q1 [easy]

Using the weighted-positional definition, convert (1101001.0110)₂ to decimal. Give the value only.

<!-- answer:start -->
exact
105.375
105.375₁₀
<!-- answer:end -->
<!-- explanation:start -->
Weights: the dot sits after position 0. Bits to the left are 1,1,0,1,0,0,1 at positions 6,5,4,3,2,1,0 and bits to the right are 0,1,1,0 at positions −1,−2,−3,−4.
Value = 1×2⁶ + 1×2⁵ + 0×2⁴ + 1×2³ + 0×2² + 0×2¹ + 1×2⁰ + 0×2⁻¹ + 1×2⁻² + 1×2⁻³ + 0×2⁻⁴
= 64 + 32 + 0 + 8 + 0 + 0 + 1 + 0 + 0.25 + 0.125 + 0 = 105.375.
Check: 105.375 in binary is 1101001.011 (trailing 0 added to match the given 4 fraction bits).
<!-- explanation:end -->

## q2 [medium]

Convert 43.6875₁₀ to binary. Show the repeated-division working for the whole part and the repeated-multiplication working for the fraction, and state the final answer.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Whole part, repeated division by 2 (prepend each remainder, read bottom-up):
43/2 = 21 rem 1; 21/2 = 10 rem 1; 10/2 = 5 rem 0; 5/2 = 2 rem 1; 2/2 = 1 rem 0; 1/2 = 0 rem 1. Reading the remainders from the last to the first: 101011.
Fraction 0.6875, repeated multiplication by 2 (append each truncated whole part):
0.6875×2 = 1.375 → 1; 0.375×2 = 0.75 → 0; 0.75×2 = 1.5 → 1; 0.5×2 = 1.0 → 1; fraction now 0, stop. Digits 1,0,1,1.
Answer: (101011.1011)₂.
Verification: 101011₂ = 32+8+2+1 = 43; 0.1011₂ = 0.5+0.125+0.0625 = 0.6875. Total 43.6875. Correct.
<!-- explanation:end -->

## q3 [medium]

A student converts 0.1₁₀ to binary using the repeated-multiplication-by-2 algorithm, expecting the loop to stop when the fraction becomes 0. It never stops. Explain why, showing the sequence of `num` values, and state the rule the lecture notes give to force termination.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Sequence: 0.1×2 = 0.2 (digit 0); 0.2×2 = 0.4 (0); 0.4×2 = 0.8 (0); 0.8×2 = 1.6 (1, fraction 0.6); 0.6×2 = 1.2 (1, fraction 0.2) — and 0.2 has been seen before, so the state 0.2 → 0.4 → 0.8 → 0.6 → 0.2 cycles forever. Because the algorithm is deterministic, once a `num` value repeats the algorithm can never reach 0, so the binary expansion of 0.1 is non-terminating and the 4-step cycle 0.2 → 0.4 → 0.8 → 0.6 repeats forever.
The notes' termination rule: limit the fraction to at most 10 digits, accepting an approximation. (Equivalently, stop when the required precision is reached.) Note the whole-part algorithm always terminates at quotient 0; only the fractional algorithm can fail to terminate.
<!-- explanation:end -->

## q4 [easy]

Convert (6204.12)₈ to decimal. Give the value only.

<!-- answer:start -->
exact
3204.15625
3204.15625₁₀
<!-- answer:end -->
<!-- explanation:start -->
Weights are powers of 8, dot after position 0: 6×8³ + 2×8² + 0×8¹ + 4×8⁰ + 1×8⁻¹ + 2×8⁻²
= 6×512 + 2×64 + 0 + 4 + 1/8 + 2/64 = 3072 + 128 + 4 + 0.125 + 0.03125 = 3204.15625.
Note: the source's printed "Steps" line shows (6224.12)₈, which is a typo — the terms it evaluates correspond to 6204.
<!-- explanation:end -->

## q5 [medium]

(a) Convert (10110010101.0100101)₂ to octal using the grouping shortcut. (b) Convert the same binary number to hexadecimal using the grouping shortcut. Show the grouped digits.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) Octal — group 3 bits from the dot, padding the ends:
whole 10110010101 → pad left to 010 110 010 101 → 2, 6, 2, 5
fraction 0100101 → pad right to 010 010 100 → 2, 2, 4
Answer: **(2625.224)₈**.

(b) Hex — group 4 bits from the dot, padding the ends:
whole 10110010101 → pad left to 0101 1001 0101 → 5, 9, 5
fraction 0100101 → pad right to 0100 1010 → 4, A
Answer: **(595.4A)₁₆** (note the fraction pads to `0100 1010`, not `0100 1000`).

Verification of the fractional part: 224₈ = 2/8 + 2/64 + 4/512 = 0.25 + 0.03125 + 0.0078125 = 0.2890625, and 4A₁₆ = 4/16 + 10/256 = 0.25 + 0.0390625 = 0.2890625. Both agree, and the whole numbers are 1429 in both bases (2625₈ = 2×512+6×64+2×8+5 = 1429; 595₁₆ = 5×256+9×16+5 = 1429). Since 3 and 4 are both powers of 2, going through binary in either direction is valid.
<!-- explanation:end -->

## q6 [medium]

Convert (21.02)₈ to binary, and (7345)₈ to hexadecimal. For the second conversion, go through binary and show both groupings.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(21.02)₈: each octal digit becomes exactly 3 bits. 2 → 010, 1 → 001, 0 → 000, 2 → 010. Concatenate: 010 001 . 000 010, then drop leading/trailing zeros: 10001.00001.
Verification: 10001₂ = 17, 00001₂ = 1/32 = 0.03125; 21.02₈ = 2×8 + 1 + 0/8 + 2/64 = 17 + 0.03125 = 17.03125. Correct.

(7345)₈ → binary: 7 → 111, 3 → 011, 4 → 100, 5 → 101, so 111011100101₂.
Regroup into 4-bit nibbles from the right: 1110 1110 0101 → E, E, 5. Leading zeros are not needed here (12 bits = 3 nibbles exactly).
Answer: **(EE5)₁₆**.
Verification: 7345₈ = 7×512 + 3×64 + 4×8 + 5 = 3584 + 192 + 32 + 5 = 3813. EE5₁₆ = 14×256 + 14×16 + 5 = 3584 + 224 + 5 = 3813. ✓
<!-- explanation:end -->

## q7 [medium]

You must convert (CA.FE)₁₆ to base 4. Why is a direct shortcut possible, and what is the result? Show how each hex digit becomes base-4 digits.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
A direct shortcut is possible because 16 = 4²: base 16 is exactly the square of base 4, so one hex digit corresponds to exactly two base-4 digits (the "other cases" generalisation: base R → base Rⁿ uses groups of n digits).
C = 12 = 1100₂ = 11 00₂ grouped in pairs = 30₄; A = 10 = 1010₂ = 10 10₂ = 22₄; F = 15 = 1111₂ = 11 11₂ = 33₄; E = 14 = 1110₂ = 11 10₂ = 32₄.
So (CA.FE)₁₆ = **(3022.3332)₄**.
Verification: 3022.3332₄ = 3×64 + 0 + 2×4 + 2 + 3/4 + 3/16 + 3/64 + 2/256 = 192+8+2 + 0.75+0.1875+0.046875+0.0078125 = 202.9921875, which equals CA.FE₁₆ = 192+10+0.9375+0.0546875 = 202.9921875. ✓
Alternative route: CA.FE = 1100 1010 . 1111 1110, regroup as 11 00 10 10 . 11 11 11 10 = 3 0 2 2 . 3 3 3 2.
<!-- explanation:end -->

## q8 [easy]

What is the smallest base in which the digit string `2100` is a valid number, and what is its decimal value in that base?

<!-- answer:start -->
exact
base 3, 63
3, 63
base 3 = 63
<!-- answer:end -->
<!-- explanation:start -->
A digit d requires base > d, so the base must exceed the largest digit present, which is 2. The smallest valid base is therefore **3**.
(2100)₃ = 2×3³ + 1×3² + 0×3¹ + 0×3⁰ = 2×27 + 1×9 = 54 + 9 = **63**.
(Related exercise from the notes, for contrast: the largest base whose value stays ≤ 1000 is 7, since (2100)₇ = 2×343 + 1×49 = 735 but (2100)₈ = 2×512 + 1×64 = 1088.)
<!-- explanation:end -->

## q9 [medium]

Convert the decimal whole number 2100 to base 3 using repeated division. Show every quotient and remainder, state the final answer, and identify which remainder is the MSB and which is the LSB.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Repeated division by 3, prepending each remainder:
2100/3 = 700 rem 0
700/3 = 233 rem 1
233/3 = 77 rem 2
77/3 = 25 rem 2
25/3 = 8 rem 1
8/3 = 2 rem 2
2/3 = 0 rem 2 → stop at quotient 0
Remainders in order: 0, 1, 2, 2, 1, 2, 2. Reading from last to first (bottom-up): **(2212210)₃**.
The last remainder produced (2) is the most significant digit (MSB); the first remainder produced (0) is the least significant digit (LSB).
Verification: 2×729 + 2×243 + 1×81 + 2×27 + 2×9 + 1×3 + 0 = 1458+486+81+54+18+3+0 = 2100. ✓
<!-- explanation:end -->

## q10 [easy]

Decode the 4-bit sign-and-magnitude patterns (a) `1101` and (b) `1001`. Give the two decimal values.

<!-- answer:start -->
exact
-5 and -1
-5, -1
(a) -5 (b) -1
<!-- answer:end -->
<!-- explanation:start -->
In sign-and-magnitude the left-most bit is the sign (1 = negative) and the remaining bits are the magnitude.
`1101`: sign 1 (negative), magnitude 101₂ = 5 ⇒ **−5**.
`1001`: sign 1 (negative), magnitude 001₂ = 1 ⇒ **−1**.
(Note `1000` in sign-and-magnitude is −0, which is a second representation of zero.)
<!-- explanation:end -->

## q11 [hard]

A student claims: "With 4-bit sign-and-magnitude I can add two negative numbers by simply doing ordinary binary addition on the 4-bit patterns." Test this claim on (−5) + (−1) using the patterns from the previous question, compute the ordinary binary sum, state what the sum decodes to, and say whether the claim is true. Then give the correct binary sum the representation should produce.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
4-bit patterns: −5 = `1101`, −1 = `1001`. Ordinary binary addition: 1101 + 1001 = 10110, truncated to 4 bits = `0110`.
Decoding `0110` in sign-and-magnitude: sign 0 (positive), magnitude 110₂ = 6 ⇒ **+6**, but the correct answer is −6. So the claim is **false**: sign-and-magnitude addition requires sign logic (add magnitudes when signs agree, subtract the smaller magnitude from the larger and take the larger's sign when they differ), not plain binary addition. This is exactly why sign-and-magnitude is listed as "-" (too complex) for addition in the operations summary.
Correct answer for (−5) + (−1) should be the pattern for −6, i.e. `1110`.
<!-- explanation:end -->

## q12 [hard]

A system stores n-bit 2s complement integers. (a) State the largest and smallest values for n = 12 in decimal. (b) Is −2048 representable in 12 bits? Give its 12-bit pattern. (c) Is 2012 representable in 12 bits?

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) For n bits, 2s complement has largest = +2ⁿ⁻¹ − 1 and smallest = −2ⁿ⁻¹. With n = 12: largest = 2¹¹ − 1 = 2048 − 1 = **2047**; smallest = −2¹¹ = **−2048**.
(b) Yes. −2048 is exactly the smallest value, whose pattern is `1000 0000 0000` (a 1 followed by eleven 0s). It is representable only because 2s complement drops the redundant −0: the codeword that would be −0 is reassigned to −2048.
(c) 2012 ≤ 2047 and ≥ −2048, so it is representable. Check: 2012 = 1024 + 512 + 256 + 128 + 64 + 16 + 8 + 4 = 11111011100₂ (11 bits); padded to 12 bits with a leading 0: `0111 1101 1100`.
<!-- explanation:end -->

## q13 [medium]

Perform the following additions in 4-bit 2s complement. Show the raw binary addition and the truncation, give the final decoded value, and say whether overflow occurred: (a) 5 + 6, (b) 4 + (−7).

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) 5 = `0101`, 6 = `0110`. Raw binary addition: `0101` + `0110` = `1011` (4 bits, no truncation needed). `1011` decodes as −8 + 2 + 1 = **−5**. Both operands are positive but the result is negative (5 + 6 = 11 is outside the 4-bit range −8…+7), so **overflow occurred**.
(b) 4 = `0100`, −7 = `1001`. Raw sum: `0100` + `1001` = `1101`. `1101` decodes as −8 + 4 + 1 = **−3**, which is the correct value (4 + (−7) = −3). The operands have different signs, so overflow can never occur here. **No overflow.**
Rule used: overflow is only possible when both operands share a sign bit; it has occurred iff the result's sign bit differs from theirs.
<!-- explanation:end -->

## q14 [medium]

In 4-bit 2s complement, compute (a) 5 + (−4) and (b) −6 + 3. Show the binary working and any truncation, state each result in decimal, and state whether overflow occurred.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) 5 = `0101`, −4 = `1100`. Raw sum: `0101` + `1100` = `10001` (5 bits). Truncate to 4 bits ⇒ `0001`, which decodes to **+1**, the correct value (5 + (−4) = 1). The operands have different signs, so overflow is impossible; **no overflow**. Note the throw-away carry out of the sign bit is not an overflow indicator.
(b) −6 = `1010`, +3 = `0011`. Raw sum `1010` + `0011` = `1101`; no truncation needed. `1101` decodes to −8 + 4 + 1 = **−3**, correct (−6 + 3 = −3). Again the operands have different signs ⇒ **no overflow**.
General rule: overflow requires both operands to share a sign bit and the result to have the opposite sign; a different-sign addition can never overflow.
<!-- explanation:end -->

## q15 [medium]

In 4-bit 1s complement, compute 3 + 5. Show the 4-bit patterns, the binary addition, any end-around carry, the final 4-bit result, the decoded value, and whether overflow occurred. Then compute (−2) + (−5) the same way and contrast the two.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
The 4-bit 1s complement range is −7…+7, so 3 + 5 = 8 is outside the range and must overflow. Working: 3 = `0011`, 5 = `0101`; `0011` + `0101` = `1000` (4 bits, so no end-around carry step is triggered and no truncation is needed). `1000` in 1s complement is **−7**. The operands are both positive but the result is negative, so **overflow occurred**.
Contrast: (−2) + (−5) = −7, which is representable. −2 = `1101`, −5 = `1010`; raw sum `1101` + `1010` = `10111`, which needs 5 bits, so the end-around carry applies: drop the carry, keep `0111`, then add 1 ⇒ `1000`, which decodes to **−7**. Both operands are negative and the final result is negative ⇒ **no overflow**.
Procedure summary: add; if the raw sum needs n+1 bits, add the carry back in (end-around carry); truncate to n bits; then check overflow by comparing result sign with operand signs.
<!-- explanation:end -->

## q16 [easy]

Consider the 4-bit Excess-8 representation. (a) Decode the pattern `0110`. (b) Encode the value 3. (c) State the range of values representable, and (d) explain why N = 8 (rather than 7) gives an even split between positive and negative values.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
In Excess-N the value of a pattern is its unsigned binary value minus N.
(a) `0110` = 6 unsigned, so value = 6 − 8 = **−2**.
(b) For value 3, the pattern's unsigned value must be 3 + 8 = 11 = `1011`. So 3 is encoded as **`1011`**.
(c) The codewords `0000`…`1111` have unsigned values 0…15, so the values are −8…+7 ⇒ range **−8 to +7**.
(d) Excess-8 maps the pattern `1000` to 0. The negatives are `0000`…`0111` (−8…−1, eight values) and the non-negatives are `1000`…`1111` (0…+7, eight values). The source's general rule for an even split is N = 2ⁿ⁻¹ = 2³ = 8 for n = 4 bits (giving −2ⁿ⁻¹ … 2ⁿ⁻¹−1). Using N = 7 instead would give −7…+8, which favours positives; the source notes that off-by-one choice is usually acceptable, and that IEEE 754 uses the 2ⁿ⁻¹−1 form for exponents.
<!-- explanation:end -->

## q17 [medium]

The 4-bit pattern `1000` is decoded in four different representations. Give the decimal value it represents in each of: (a) sign-and-magnitude, (b) 1s complement, (c) 2s complement, (d) Excess-8.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) Sign-and-magnitude: sign bit 1 = negative, magnitude `000` = 0 ⇒ **−0** (the negative zero; +0 is `0000`, so this representation wastes a codeword).
(b) 1s complement: flip all bits of `1000` to negate/read it — or use the 4-bit table: `1000` = **−7**.
(c) 2s complement: `1000` is the most negative value, **−8** (there is no −0 in 2s complement; the codeword that was −0 in 1s complement became −1, shifting all negatives down by one, so here it is −8).
(d) Excess-8: unsigned 8 ⇒ 8 − 8 = **0**.
This one pattern therefore means −0, −7, −8 and 0 respectively — a good reminder that a bit pattern is meaningless without stating the representation.
<!-- explanation:end -->

## q18 [hard]

Represent −55₁₀ using 8 bits in each of: (a) sign-and-magnitude, (b) 1s complement, (c) 2s complement, (d) Excess-128. Show the working for each and state the four 8-bit patterns.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
First, 55₁₀ = 32 + 16 + 4 + 2 + 1 = `00110111` in 8 bits.
(a) Sign-and-magnitude: flip the left-most (sign) bit to 1 ⇒ **`10110111`**.
(b) 1s complement: flip all bits of `00110111` ⇒ **`11001000`**. (Formula: 2⁸ − 55 − 1 = 200 = 11001000.)
(c) 2s complement: from the 1s answer add 1 ⇒ `11001000` + 1 = **`11001001`**. (Formula: 2⁸ − 55 = 201 = 11001001.)
(d) Excess-128: pattern `00000000` represents the starting number −128, so a value v is encoded as the unsigned pattern v + 128. For v = −55: −55 + 128 = 73 = 64 + 8 + 1 = `01001001`. The notes state it as 128 − 55 = 73 = `01001001`. Check by decoding: 73 − 128 = −55 ✓.
Final answers: (a) `10110111`, (b) `11001000`, (c) `11001001`, (d) `01001001`.
Cross-check the three complement forms are consistent: 1s complement is the bit-flip of the positive magnitude, and 2s complement is the 1s pattern plus 1 (`11001000` + 1 = `11001001`).
<!-- explanation:end -->

## q19 [easy]

Encode −5 using 4 bits in (a) 1s complement, (b) 2s complement, and (c) Excess-8. Show the working for each.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
First, 5₁₀ = `0101` in 4 bits.
(a) 1s complement: flip all bits ⇒ **`1010`**. Formula check: 2⁴ − 5 − 1 = 10 = 1010. ✓
(b) 2s complement: flip all bits then add 1: `1010` + 1 = **`1011`**. Formula check: 2⁴ − 5 = 11 = 1011. ✓
(c) Excess-8: the pattern's unsigned value must be 5 less than 8, i.e. 8 − 5 = 3 = `0011`. So **`0011`**. Check: 3 − 8 = −5. ✓
Note (a) and (b) differ by exactly one in magnitude, which is the shift caused by removing the redundant −0.
<!-- explanation:end -->

## q20 [easy]

A fixed-point format allocates 10 bits to the whole-number part and 6 bits to the fraction part. (a) What is the resolution? (b) Show that 3.64 is not exactly representable. (c) Give the value obtained if the representation is truncated to the available fraction bits, and (d) state the approximation error.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) Resolution = 2⁻ᶠ with f = 6 ⇒ 2⁻⁶ = **0.015625**. Every representable value is an integer multiple of 0.015625.
(b) 3.64 / 0.015625 = 232.96, which is not an integer, so 3.64 is not a multiple of the resolution and cannot be represented exactly.
(c) Truncation keeps the whole-number multiple: ⌊232.96⌋ = 232, and 232 × 0.015625 = **3.625**.
(d) Error = 3.64 − 3.625 = **0.015** (the truncated value is below the true value by 0.015). Note the maximum truncation error for this format is 0.015625 (one whole resolution step), and the notes state that truncation is the usual practical policy.
<!-- explanation:end -->

## q21 [hard]

Convert −36.03125₁₀ to a 16-bit fixed-point 1s complement number with 10 whole-number bits and 6 fraction bits. Show the conversion of the magnitude, the padding, and the negation, and give the final bit pattern.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Step 1 — magnitude to binary. Whole part 36 = `100100`. Fraction 0.03125 = 1/32 = 2⁻⁵ = `00001` (check: 0.03125×2 = 0.0625→0; ×2 = 0.125→0; ×2 = 0.25→0; ×2 = 0.5→0; ×2 = 1.0→1, so digits 00001). So 36.03125 = `100100.00001`₂.
Step 2 — pad to 10 whole + 6 fraction bits: whole `100100` (6 bits) needs 4 leading zeros ⇒ `0000100100`; fraction `00001` (5 bits) needs 1 trailing zero ⇒ `000010`.
So the positive pattern is `0000100100.000010`.
Step 3 — negate in 1s complement by inverting all bits: `1111011011.111101`.
Final answer: **1111011011.111101₁s**. (Cross-check with the formula −x = 2ⁿ − x − 2⁻ᶠ = 1024 − 36.03125 − 0.015625 = 987.953125; and 1111011011.111101₂ = 987 + 0.953125 = 987.953125 ✓.)
<!-- explanation:end -->

## q22 [medium]

Encode −6.5₁₀ in IEEE 754 single precision. Show the binary conversion, normalisation, the sign bit, the excess-127 exponent, the 23-bit mantissa, and give the final answer in hexadecimal.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
Step 1: 6.5 = 110.1₂, so −6.5 = −110.1₂.
Step 2: normalise keeping the hidden bit: −110.1₂ = −1.101₂ × 2².
Step 3: sign = 1 (negative).
Step 4: exponent = 2 + 127 = 129 = `10000001` (8-bit Excess-127).
Step 5: mantissa = the 3 fraction bits `101`, padded with zeros to 23 bits: `10100000000000000000000`.
Concatenate: 1 | 10000001 | 10100000000000000000000.
Group into nibbles: 1100 0001 1101 0000 0000 0000 0000 0000 = C 0 D 0 0 0 0 0.
Final answer: **0xC0D00000**. (The leading 1 of 1.101 is the hidden bit and is not stored.)
<!-- explanation:end -->

## q23 [hard]

Decode the IEEE 754 single-precision bit pattern 0x41A00000. Show the field split, the exponent, the normalised mantissa with the hidden bit restored, and the final decimal value.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
0x41A00000 = 0100 0001 1010 0000 0000 0000 0000 0000 in binary.
Field split (1 sign | 8 exponent | 23 mantissa): sign = `0`; exponent = `10000011`; mantissa = `01000000000000000000000`.
Exponent: `10000011` = 128 + 2 + 1 = 131. Since the exponent is stored in Excess-127, the true exponent = 131 − 127 = **4**.
Mantissa: restore the hidden bit ⇒ 1.01000000000000000000000₂ = 1 + 0×2⁻¹ + 1×2⁻² = 1.25.
Value = (−1)^0 × 1.25 × 2⁴ = 1.25 × 16 = **20.0**.
Check by direct encoding: 20 = 10100₂ = 1.01₂ × 2⁴; exponent stored = 4 + 127 = 131 = `10000011`; mantissa = `010…0`; sign 0. ✓
<!-- explanation:end -->

## q24 [hard]

Explain, in terms of the normalised mantissa, (a) why the "hidden bit" trick works in binary but not in decimal scientific notation, and (b) why the value 0 cannot be represented by the normalised form at all.

<!-- answer:start -->
manual
<!-- answer:end -->
<!-- explanation:start -->
(a) Normalising means shifting the binary point until there is exactly one non-zero digit in front of it. In binary there are only two symbols, 0 and 1, so a non-zero normalised mantissa must begin with 1 — there is no other possibility (anything of the form 0._ would be denormalised/zero). Therefore the leading 1 can be *assumed* rather than stored, gaining one bit of precision for free. In decimal there are nine possible non-zero leading digits (1–9), so a decimal normalised mantissa of the form "d.xxxx" would still need to record d; the trick does not apply.
(b) The normalised form is defined to be 1.<fraction>; the value 0 has no leading 1 (and no exponent that makes it non-zero), so it cannot be written in normalised form. IEEE 754 therefore reserves the all-zero exponent field (with an all-zero mantissa) as a special case meaning ±0 (and uses other reserved exponent values for denormals, infinity and NaN). The lecture notes cover the normalised-mantissa and Excess-exponent mechanics but do not spell out these special-case encodings.
<!-- explanation:end -->

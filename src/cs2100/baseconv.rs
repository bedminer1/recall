//! CS2100 — number-system helper (NUS CS2100, data representation).
//!
//! Converts between decimal, hexadecimal, binary and octal, and shows the three
//! signed representations this course uses: two's complement, one's complement
//! and signed magnitude. Negative values also print their sign extension, which
//! is what MIPS immediates rely on.
//!
//! Wired into the CLI as `recall baseconv`:
//!
//! ```text
//! recall baseconv 0b0110010          # 50, 0x32
//! recall baseconv 0xFB -b 8          # that pattern read as 251 / -5 / -4 / -123
//! recall baseconv -5 -b 8 -b 16      # encode -5, plus sign extension
//! recall baseconv -i bin "1010 1111" # force the input base, spaces allowed
//! ```

/// The signed representations shown for an n-bit pattern.
const SCHEMES: [&str; 3] = ["two's complement", "one's complement", "signed magnitude"];

/// Entry point for `recall baseconv <value> [-b N]... [-i base]`.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut widths: Vec<u32> = Vec::new();
    let mut in_base: Option<String> = None;
    let mut value: Option<String> = None;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "-b" | "--bits" => {
                index += 1;
                match args.get(index).and_then(|raw| raw.parse::<u32>().ok()) {
                    Some(bits) if (1..=64).contains(&bits) => widths.push(bits),
                    _ => return Err("baseconv: -b needs a bit width between 1 and 64".into()),
                }
            }
            "-i" | "--in-base" => {
                index += 1;
                match args.get(index) {
                    Some(base) => in_base = Some(base.to_ascii_lowercase()),
                    None => return Err("baseconv: -i needs one of bin/oct/dec/hex".into()),
                }
            }
            other => value = Some(other.to_string()),
        }
        index += 1;
    }

    let Some(text) = value else {
        print_help();
        return Ok(());
    };

    let parsed = parse_value(&text, in_base.as_deref())?;
    if widths.is_empty() {
        show_plain(parsed);
    } else {
        for bits in &widths {
            show_width(parsed, *bits);
        }
    }
    Ok(())
}

fn print_help() {
    println!(
        "baseconv - CS2100 number-system conversions\n\n\
         usage:\n\
           recall baseconv <value> [-b BITS]... [-i BASE]\n\n\
         value:\n\
           255 | -5 | 0xFF | 0b1010 | 0o17 | 0d42\n\n\
         options:\n\
           -b, --bits N   show N-bit representations (repeatable, 1..=64)\n\
           -i, --in-base  force the input base: bin | oct | dec | hex\n\n\
         examples:\n\
           recall baseconv 0b0110010\n\
           recall baseconv 0xFB -b 8\n\
           recall baseconv -5 -b 8 -b 16\n\
           recall baseconv -i bin \"1010 1111\""
    );
}

/// Parse `255`, `-5`, `0xFF`, `0b1010`, `0o17`, `0d42`. Spaces, commas and
/// underscores are ignored, so spaced binary pasted from notes still works.
fn parse_value(text: &str, force: Option<&str>) -> Result<i128, String> {
    let cleaned: String = text
        .chars()
        .filter(|c| !matches!(c, '_' | ',' | ' '))
        .collect();
    let (negative, body) = match cleaned.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, cleaned.strip_prefix('+').unwrap_or(cleaned.as_str())),
    };
    let body = body.to_ascii_lowercase();

    let (digits, radix) = match force {
        Some("bin") => (body.as_str(), 2),
        Some("oct") => (body.as_str(), 8),
        Some("dec") => (body.as_str(), 10),
        Some("hex") => (body.as_str(), 16),
        Some(other) => {
            return Err(format!(
                "baseconv: unknown base {other:?} (use bin/oct/dec/hex)"
            ));
        }
        None if body.starts_with("0x") => (&body[2..], 16),
        None if body.starts_with("0b") => (&body[2..], 2),
        None if body.starts_with("0o") => (&body[2..], 8),
        None if body.starts_with("0d") => (&body[2..], 10),
        None => (body.as_str(), 10),
    };

    let magnitude = i128::from_str_radix(digits, radix)
        .map_err(|_| format!("baseconv: cannot parse {text:?}"))?;
    Ok(if negative { -magnitude } else { magnitude })
}

fn mask_of(bits: u32) -> u128 {
    (1u128 << bits) - 1
}

/// The low `bits` bits of `value`, grouped in nibbles for readability.
fn bits_str(value: i128, bits: u32) -> String {
    let masked = (value as u128) & mask_of(bits);
    let binary = format!("{:0width$b}", masked, width = bits as usize);
    binary
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(" ")
}

fn hex_str(value: i128, bits: u32) -> String {
    let digits = (bits as usize).div_ceil(4);
    format!(
        "0x{:0width$X}",
        (value as u128) & mask_of(bits),
        width = digits
    )
}

/// The n-bit code for `value` under two's complement, one's complement and
/// signed magnitude.
fn encode(value: i128, bits: u32) -> [u128; 3] {
    let mask = mask_of(bits);
    let negative = value < 0;
    let magnitude: u128 = if negative {
        (-value) as u128
    } else {
        value as u128
    };
    let twos = (value as u128) & mask;
    let ones = if negative {
        (mask ^ magnitude) & mask
    } else {
        magnitude & mask
    };
    let signed_magnitude = if negative {
        ((1u128 << (bits - 1)) | magnitude) & mask
    } else {
        magnitude & mask
    };
    [twos, ones, signed_magnitude]
}

/// An n-bit pattern read as unsigned, two's complement, one's complement and
/// signed magnitude.
fn decode(code: u128, bits: u32) -> [i128; 4] {
    let mask = mask_of(bits);
    let code = code & mask;
    let unsigned = code as i128;
    let is_negative = (code >> (bits - 1)) & 1 == 1;
    let magnitude = (code & ((1u128 << (bits - 1)) - 1)) as i128;
    let twos = if is_negative {
        unsigned - (1i128 << bits)
    } else {
        unsigned
    };
    let ones = if is_negative {
        -(((mask ^ code) & mask) as i128)
    } else {
        unsigned
    };
    let signed_magnitude = if is_negative { -magnitude } else { unsigned };
    [unsigned, twos, ones, signed_magnitude]
}

fn fits(value: i128, scheme: usize, bits: u32) -> bool {
    let high = (1i128 << (bits - 1)) - 1;
    if scheme == 0 {
        -(1i128 << (bits - 1)) <= value && value <= high
    } else {
        -high <= value && value <= high
    }
}

fn show_plain(value: i128) {
    println!("dec {value}");
    if value >= 0 {
        println!("hex {value:X}");
        println!("bin {value:b}");
        println!("oct {value:o}");
    } else {
        println!(
            "hex -{:X}   (negative: add -b N for an n-bit pattern)",
            -value
        );
        println!("bin -{:b}", -value);
        println!("oct -{:o}", -value);
    }
    println!();
}

fn show_width(value: i128, bits: u32) {
    let mask = mask_of(bits);
    println!(
        "--- {value} in {bits} bits (two's complement range {} .. {}) ---",
        -(1i128 << (bits - 1)),
        (1i128 << (bits - 1)) - 1
    );

    // A non-negative value that fits is a pattern to read, not a value to encode.
    if value >= 0 && (value as u128) <= mask {
        let [unsigned, twos, ones, signed_magnitude] = decode(value as u128, bits);
        println!(
            "pattern {}  {} reads as:",
            hex_str(value, bits),
            bits_str(value, bits)
        );
        println!("  unsigned          {unsigned}");
        println!("  two's complement  {twos}");
        println!("  one's complement  {ones}");
        println!("  signed magnitude  {signed_magnitude}");
        println!();
        return;
    }

    let codes = encode(value, bits);
    println!(
        "{:<18}{:<8}{:<22}{:<10}",
        "scheme", "hex", "binary", "unsigned"
    );
    for (index, scheme) in SCHEMES.iter().enumerate() {
        let note = if fits(value, index, bits) {
            ""
        } else {
            "  <-- not representable here"
        };
        println!(
            "{:<18}{:<8}{:<22}{:<10}{}",
            scheme,
            hex_str(codes[index] as i128, bits),
            bits_str(codes[index] as i128, bits),
            codes[index],
            note
        );
    }

    if !fits(value, 0, bits) {
        println!(
            "note: {value} does not fit in {bits} signed bits; patterns show value mod 2^{bits}"
        );
    }

    for wider in [16u32, 32] {
        if wider > bits {
            println!(
                "sign-extended to {wider:>2} bits: {} {}",
                hex_str(value, wider),
                bits_str(value, wider)
            );
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_documented_prefix() {
        assert_eq!(parse_value("0b0110010", None).unwrap(), 50);
        assert_eq!(parse_value("0x1D", None).unwrap(), 29);
        assert_eq!(parse_value("0o62", None).unwrap(), 50);
        assert_eq!(parse_value("0d50", None).unwrap(), 50);
        assert_eq!(parse_value("-5", None).unwrap(), -5);
        assert_eq!(parse_value("1010 1111", Some("bin")).unwrap(), 175);
        assert_eq!(parse_value("1_0000", None).unwrap(), 10_000);
        assert!(parse_value("zzz", None).is_err());
    }

    #[test]
    fn encodes_negative_five_in_eight_bits() {
        let [twos, ones, signed_magnitude] = encode(-5, 8);
        assert_eq!(twos, 0xFB); // 1111 1011
        assert_eq!(ones, 0xFA); // 1111 1010
        assert_eq!(signed_magnitude, 0x85); // 1000 0101
    }

    #[test]
    fn reads_one_pattern_four_ways() {
        let [unsigned, twos, ones, signed_magnitude] = decode(0xFB, 8);
        assert_eq!(unsigned, 251);
        assert_eq!(twos, -5);
        assert_eq!(ones, -4);
        assert_eq!(signed_magnitude, -123);
    }

    #[test]
    fn sign_extends_negative_values() {
        assert_eq!(hex_str(-5, 16), "0xFFFB");
        assert_eq!(hex_str(-5, 32), "0xFFFFFFFB");
        assert_eq!(bits_str(-5, 8).replace(' ', ""), "11111011");
        assert_eq!(
            bits_str(-5, 32).replace(' ', ""),
            format!("{:032b}", 0xFFFF_FFFBu32)
        );
    }

    #[test]
    fn reports_what_each_scheme_can_hold() {
        assert!(fits(-128, 0, 8));
        assert!(!fits(-129, 0, 8));
        assert!(fits(-127, 1, 8));
        assert!(!fits(-128, 1, 8)); // one's complement has no negative zero
        assert!(fits(127, 2, 8));
        assert!(!fits(128, 2, 8));
    }
}

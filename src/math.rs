#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// Returns the lowest common multiple of two numbers.
pub fn lcm(a: usize, b: usize) -> usize {
    if a == 0 && b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        b => gcd(b, a % b),
    }
}

/// What the fuck
///
/// https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation/
pub fn chinese_remainder(numbers: &[isize], remainders: &[isize]) -> Result<isize, String> {
    if numbers.len() != remainders.len() {
        return Err(format!(
            "Incompatible number-remainder array length: {} vs {}",
            numbers.len(),
            remainders.len()
        ));
    }

    let product = numbers.iter().fold(1, |prev, &n| prev * n);

    (0..numbers.len())
        .try_fold(0, |prev, i| {
            let &number = unsafe { numbers.get_unchecked(i) };
            let &remainder = unsafe { remainders.get_unchecked(i) };

            let division = product / number;
            let mi = mod_inverse(division, number).ok_or_else(|| {
                format!(
                    "Couldn't find multiplicative inverse modulo for (a={}, m={})",
                    division, number
                )
            })?;

            Ok(prev + remainder * mi * division)
        })
        .map(|result| result % product)
}

/// What the fuck
///
/// https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
pub fn mod_inverse(a: isize, m: isize) -> Option<isize> {
    match gcd_extended(a, m) {
        (1, x, _) => Some((x % m + m) % m),
        (_, _, _) => None,
    }
}

/// What the fuck
///
/// https://www.geeksforgeeks.org/euclidean-algorithms-basic-and-extended/
pub fn gcd_extended(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x, y) = gcd_extended(b % a, a);
        (gcd, y - (b / a) * x, x)
    }
}

pub fn f2i(x: f64) -> i64 {
    // note that this compiles down to
    // movq    rax, xmm0
    i64::from_le_bytes(x.to_le_bytes())
}

pub fn i2f(x: i64) -> f64 {
    // note that this compiles down to
    // movq    xmm0, rdi
    f64::from_le_bytes(x.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f2i_i2f() {
        let x = 123.456;
        let y = f2i(x);
        let z = i2f(y);
        assert_eq!(x, z);

        let x = -123.456;
        let y = f2i(x);
        let z = i2f(y);
        assert_eq!(x, z);

        let x = 0.0;
        let y = f2i(x);
        let z = i2f(y);
        assert_eq!(x, z);

        let x = 1.0;
        let y = f2i(x);
        let z = i2f(y);
        assert_eq!(x, z);

        let x = -1.0;
        let y = f2i(x);
        let z = i2f(y);
        assert_eq!(x, z);
    }

    #[test]
    fn test_i2f_f2i() {
        let x = 123;
        let y = i2f(x);
        let z = f2i(y);
        assert_eq!(x, z);

        let x = -123;
        let y = i2f(x);
        let z = f2i(y);
        assert_eq!(x, z);

        let x = 0;
        let y = i2f(x);
        let z = f2i(y);
        assert_eq!(x, z);

        let x = 1;
        let y = i2f(x);
        let z = f2i(y);
        assert_eq!(x, z);

        let x = -1;
        let y = i2f(x);
        let z = f2i(y);
        assert_eq!(x, z);
    }
}

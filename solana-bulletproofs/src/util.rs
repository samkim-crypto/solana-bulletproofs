use solana_ristretto::scalar::Scalar;

/// Represents a degree-1 vector polynomial `a + b * x`.
pub struct VecPoly1(pub Vec<Scalar>, pub Vec<Scalar>);

impl VecPoly1 {
    pub fn zero(n: usize) -> Self {
        VecPoly1(vec![Scalar::zero(); n], vec![Scalar::zero(); n])
    }

    pub fn inner_product(&self, rhs: &VecPoly1) -> Option<Poly2> {
        // Use Karatsuba's method
        let l = self;
        let r = rhs;

        let t0 = inner_product(&l.0, &r.0)?;
        let t2 = inner_product(&l.1, &r.1)?;

        let l0_plus_l1 = add_vec(&l.0, &l.1);
        let r0_plus_r1 = add_vec(&r.0, &r.1);

        let mut t1 = inner_product(&l0_plus_l1, &r0_plus_r1)?;
        t1 = t1.subtract_scalar(t0).unwrap();
        t1 = t1.subtract_scalar(t2).unwrap();

        Some(Poly2(t0, t1, t2))
    }

    pub fn eval(&self, x: Scalar) -> Vec<Scalar> {
        let n = self.0.len();
        let mut result = vec![Scalar::zero(); n];
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            let product = self.1[i].multiply_scalar(x).unwrap();
            result[i] = self.0[i].add_scalar(product).unwrap();
        }
        result
    }
}

/// Represents a degree-2 scalar polynomial `a + b * x + c * x^2`
pub struct Poly2(pub Scalar, pub Scalar, pub Scalar);

impl Poly2 {
    pub fn eval(&self, x: Scalar) -> Scalar {
        // compute `self.0 + x * (self.1 + x * self.2)`
        let result = x.multiply_scalar(self.2).unwrap();
        let result = result.add_scalar(self.1).unwrap();
        let result = result.multiply_scalar(x).unwrap();
        result.add_scalar(self.0).unwrap()
    }
}

/// Add the sum of two scalar vectors.
pub fn add_vec(a: &[Scalar], b: &[Scalar]) -> Vec<Scalar> {
    if a.len() != b.len() {
        panic!("lengths of vectors don't match for vector addition");
    }
    let mut result = vec![Scalar::zero(); b.len()];
    for i in 0..a.len() {
        result[i] = a[i].add_scalar(b[i]).unwrap();
    }
    result
}

/// Given `data` with `len >= 32`, return the first 32 bytes.
pub fn read32(data: &[u8]) -> [u8; 32] {
    let mut buf32 = [0u8; 32];
    buf32[..].copy_from_slice(&data[..32]);
    buf32
}

/// Computes an inner product of two scalar vectors.
pub fn inner_product(a: &[Scalar], b: &[Scalar]) -> Option<Scalar> {
    let mut result = Scalar::zero();
    if a.len() != b.len() {
        return None;
    }
    for i in 0..a.len() {
        let product = a[i].multiply_scalar(b[i]).unwrap();
        result = result.add_scalar(product).unwrap();
    }
    Some(result)
}

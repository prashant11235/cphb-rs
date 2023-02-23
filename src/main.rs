type FieldElement = [i64; 16];

/*
fn unpack25519(out: FieldElement, *in: u8) {
    int i;
    for (i = 0; i < 16; ++i) out[i] = in[2*i] + ((i64) in[2*i + 1] << 8);
    out[15] &= 0x7fff;
}

fn carry25519(elem: FieldElement) {
    int i;
    i64 carry;
    for (i = 0; i < 16; ++i) {
        carry = elem[i] >> 16;
        elem[i] -= carry << 16;
        if (i < 15) elem[i + 1] += carry; else elem[0] += 38 * carry;
    }
}
*/

/* out = a + b */
fn fadd(out: &mut FieldElement, a: FieldElement, b: FieldElement) {
    for i in 0..16 {
        out[i] = a[i] + b[i];
    }
}

/* out = a - b */
fn fsub(out: &mut FieldElement, a: FieldElement, b: FieldElement) {
    for i in 0..16 {
        out[i] = a[i] - b[i];
    }
}

/*
/* out = a * b */
fn fmul(out: FieldElement, a: FieldElement, b: FieldElement) {
    i64 i, j, product[31];
    for (i = 0; i < 31; ++i) product[i] = 0;
    for (i = 0; i < 16; ++i) {
        for (j = 0; j < 16; ++j) product[i+j] += a[i] * b[j];
    }
    for (i = 0; i < 15; ++i) product[i] += 38 * product[i + 16];
    for (i = 0; i < 16; ++i) out[i] = product[i];
    carry25519(out);
    carry25519(out);
}
*/

fn main() {
    print!("Hello");

    let a: FieldElement = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let b: FieldElement = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15];
    let mut out: FieldElement = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    fsub(&mut out, a, b);

    for i in 0..16 {
        print!("{} ", out[i]);
    }

    fadd(&mut out, a, b);
    for i in 0..16 {
        print!("{} ", out[i]);
    }
}
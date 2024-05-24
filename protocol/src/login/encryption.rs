/// https://gist.github.com/Lazersmoke/9947ada8acdd74a8b2e37d77cf1e0fdc
pub fn asn1_encode_key<M, E>(modulo: M, exponent: E) -> Vec<u8>
where
    M: core::ops::Deref<Target = openssl::bn::BigNumRef>,
    E: core::ops::Deref<Target = openssl::bn::BigNumRef>,
{
    let modulo: &openssl::bn::BigNumRef = modulo.deref();
    let exponent: &openssl::bn::BigNumRef = exponent.deref();

    let asnOIDForRSAKeys = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];

    let asnSequence = [0x30];
    let asnObjectId = [0x06];
    let asnTag = [0x05];
    let asnBitString = [0x03];
    let asnInt = [0x02];
    let nullForGoodLuck = [0x00];

    let algParams: Vec<u8> = join_vec(asnTag.to_vec(), nullForGoodLuck.to_vec());
    let algObjectId: Vec<u8> =
        join_vec(asnObjectId.to_vec(), with_length(asnOIDForRSAKeys.to_vec()));
    let algIdentifier = join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(algObjectId.clone(), algParams.clone())),
    );

    let bytesOfModulus = int_bytes_raw(modulo);
    let bytesOfExponent = int_bytes_raw(exponent);

    let theModulus = join_vec(
        asnInt.to_vec(),
        with_length(join_vec(nullForGoodLuck.to_vec(), bytesOfModulus.clone())),
    );

    let theExponent = join_vec(asnInt.to_vec(), with_length(bytesOfExponent.clone()));

    let pubKeySequence = join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(theModulus.clone(), theExponent.clone())),
    );

    let pubKeyBitString = join_vec(
        asnBitString.to_vec(),
        with_length(join_vec(nullForGoodLuck.to_vec(), pubKeySequence.clone())),
    );

    fn int_bytes_raw(v: &openssl::bn::BigNumRef) -> Vec<u8> {
        return v.to_vec();

        let mut result = Vec::new();
        let mut tmp = openssl::bn::BigNum::new().unwrap();
        let mut idx = 0;
        while tmp > openssl::bn::BigNum::new().unwrap() {
            // result.insert(0, (v & 0xff) as u8);
            tmp.rshift(&v, 8 * idx).unwrap();
            idx += 1;
        }

        result
    }

    fn join_vec(mut f: Vec<u8>, mut s: Vec<u8>) -> Vec<u8> {
        f.append(&mut s);
        f
    }

    fn len_of(v: &[u8]) -> Vec<u8> {
        if v.len() < 128 {
            return vec![v.len() as u8];
        } else {
            let len = v.len();

            let following_bytes = len.to_be_bytes();
            let first_byte =
                0b10000000 | (following_bytes.iter().skip_while(|v| **v == 0).count() as u8);

            core::iter::once(first_byte)
                .chain(following_bytes.iter().skip_while(|v| **v == 0).copied())
                .collect()
        }
    }
    fn with_length(mut v: Vec<u8>) -> Vec<u8> {
        let mut length = len_of(&v);
        length.append(&mut v);
        length
    }

    join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(algIdentifier, pubKeyBitString)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let mut modulo = openssl::bn::BigNum::new().unwrap();
        let mut exponent = openssl::bn::BigNum::new().unwrap();

        modulo.add_word(0xff);
        exponent.add_word(0x010001);

        let result = asn1_encode_key(modulo, exponent);

        assert_eq!(
            &[
                0x30, 0x1d, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01,
                0x01, 0x05, 0x00, 0x03, 0x0c, 0x00, 0x30, 0x09, 0x02, 0x02, 0x00, 0xff, 0x02, 0x03,
                0x01, 0x00, 0x01
            ],
            result.as_slice()
        );
    }
}

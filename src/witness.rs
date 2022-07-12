use num_bigint::BigInt;

pub fn witness_ints_to_field_elts<E: ark_ec::PairingEngine>(
    witness_ints: Vec<BigInt>,
) -> Vec<E::Fr> {
    // NOTE: manually pulled from ark_bn254. not ideal!
    // let modulus = BigInteger([
    //     0x3c208c16d87cfd47,
    //     0x97816a916871ca8d,
    //     0xb85045b68181585d,
    //     0x30644e72e131a029,
    // ]);
    use ark_ff::{FpParameters, PrimeField};

    let modulus = <<E::Fr as PrimeField>::Params as FpParameters>::MODULUS;

    use num_traits::Signed;
    let witness = witness_ints
        .into_iter()
        .map(|w| {
            let w = if w.sign() == num_bigint::Sign::Minus {
                // Need to negate the witness element if negative
                modulus.into() - w.abs().to_biguint().unwrap()
            } else {
                w.to_biguint().unwrap()
            };
            E::Fr::from(w)
        })
        .collect::<Vec<_>>();

    witness
}

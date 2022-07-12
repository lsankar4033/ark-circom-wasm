// use ark_std::rand::thread_rng;
mod witness;
use witness::witness_ints_to_field_elts;

use wasm_bindgen::prelude::*;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use ark_std::rand::thread_rng;

#[wasm_bindgen]
pub fn groth16_prove() {
    // let cfg = CircomConfig::<Bn254>::new(wasm_file, r1cs_file)?;
    // let mut builder = CircomBuilder::new(cfg);

    // let circom = builder.setup();
    // builder.set_witness(witness);

    // let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    // // let circom = builder.build()?;

    // let proof = prove(circom, &params, &mut rng)?;
}

use std::time::Instant;

use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::crh::bowe_hopwood::Parameters;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::EqGadget;
// TODO: 
//    1. Match API version with BH hashing in Arkworks
//    2. Call BH native and circuit function here.
use ark_std::rand::{RngCore};

use ark_crypto_primitives::crh::{bowe_hopwood, pedersen};
use ark_crypto_primitives::{CRHScheme, CRHSchemeGadget}; 
use ark_ed_on_bls12_381::{constraints::FqVar, EdwardsParameters, Fq as Fr};
use ark_r1cs_std::{alloc::AllocVar, uint8::UInt8, R1CSVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::test_rng;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Window;

impl pedersen::Window for Window {
    const WINDOW_SIZE: usize = 63;
    const NUM_WINDOWS: usize = 2;
}

type TestCRH = bowe_hopwood::CRH<EdwardsParameters, Window>;
type TestCRHGadget = bowe_hopwood::constraints::CRHGadget<EdwardsParameters, FqVar>;


fn generate_u8_input(
    cs: ConstraintSystemRef<Fr>,
    input: Vec<u8>,
) -> Vec<UInt8<Fr>> {
    let mut input_bytes = vec![];
    for byte in input.iter() {
        input_bytes.push(UInt8::new_witness(cs.clone(), || Ok(byte)).unwrap());
    }
    input_bytes
}

#[derive(Clone)]
pub struct BH {
    pub input: Vec<u8>,
    pub parameters: Parameters<EdwardsParameters>,
    pub primitive_result: Fr
    // pub output: Vec<u8>,
}

impl ConstraintSynthesizer<Fr> for BH {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let input_var = generate_u8_input(cs.clone(), self.input.clone());

        let parameters_var =
            <TestCRHGadget as CRHSchemeGadget<TestCRH, Fr>>::ParametersVar::new_witness(
                ark_relations::ns!(cs, "parameters_var"),
                || Ok(&self.parameters),
            )
            .unwrap();

        let actual_output = TestCRHGadget::evaluate(&parameters_var, &input_var).unwrap();
        let expected = FpVar::new_witness(cs.clone(), || Ok(&self.primitive_result)).unwrap();

        actual_output.enforce_equal(&expected).unwrap();

        Ok(())
    }
}


// // TODO: copy arkwork bowe_hopw code here.

fn main() {

    let size = 32; // following generate_u8_input()
    let mut rng = &mut test_rng();
    let mut input = vec![1u8; size];
    rng.fill_bytes(&mut input);

    // Get correct input and output of bowe_hopw here.

    let parameters = TestCRH::setup(rng).unwrap();
    let primitive_result = TestCRH::evaluate(&parameters, input.as_slice()).unwrap();
    
    // Get circuit here
    let bh_circuit = BH {
        input: input.clone(),
        parameters: parameters.clone(),
        primitive_result: primitive_result.clone(),
    };

    // Setup
    println!("start generating random parameters");
    let begin = Instant::now();
    let param =
        ark_groth16::generate_random_parameters::<Bls12_381, _, _>(bh_circuit.clone(), &mut rng)
            .unwrap();
    let end = Instant::now();
        println!("setup time {:?}", end.duration_since(begin));

    let pvk = ark_groth16::prepare_verifying_key(&param.vk);
    println!("random parameters generated!\n");
    
    // Prover
    let begin = Instant::now();
    let proof = ark_groth16::create_random_proof(bh_circuit, &param, &mut rng).unwrap();
    let end = Instant::now();
    println!("prove time {:?}", end.duration_since(begin));

    // Verifier
    let inputs: Vec<Fr> = vec![];
    let begin = Instant::now();
    assert!(ark_groth16::verify_proof(&pvk, &proof, &inputs[..]).unwrap());
    let end = Instant::now();
    println!("verification time {:?}", end.duration_since(begin));

    println!("Run successfully!")
}
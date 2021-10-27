// TODO: 
//    1. Match API version with BH hashing in Arkworks
//    2. Call BH native and circuit function here.
use ark_bls12_381::*;
use ark_ff::{Field, PrimeField, FpParameters, BigInteger};
use ark_std::{One, Zero, UniformRand};

use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::eq::EqGadget;
// use ark_r1cs_std::ed_on_bls12_381::FqVar;
use ark_groth16::*;
use std::time::Instant;


use ark_crypto_primitives::crh::bowe_hopwood::*;
use ark_std::rand::Rng;

// use crate::crh::bowe_hopwood;
// use crate::crh::{pedersen, TwoToOneCRHScheme, TwoToOneCRHSchemeGadget};
// use crate::{CRHScheme, CRHSchemeGadget};
use ark_ed_on_bls12_381::{constraints::FqVar, EdwardsParameters, Fq as Fr};
use ark_r1cs_std::{alloc::AllocVar, uint8::UInt8, R1CSVar};
use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};
use ark_std::test_rng;

use ark_crypto_primitives::crh::{bowe_hopwood, pedersen};
// use ark_crypto_primitives::crh::{CRHScheme, CRHSchemeGadget};
use ark_crypto_primitives::{
    crh::{
        bowe_hopwood::{Parameters, CHUNK_SIZE},
        pedersen::Window,
        // CRHSchemeGadget,
    },
};

type TestCRH = bowe_hopwood::CRH<EdwardsParameters, Window>;
// type TestCRHGadget = bowe_hopwood::CRHGadget<EdwardsParameters, FqVar>;


// fn generate_input(
//     cs: ConstraintSystemRef<Fq>,
//     input: Vec<u8>,
// ) -> Vec<FqVar> {
//     let mut res: Vec<FqVar> = vec![FpVar::<Fq>::Constant(Fq::zero()); input.len()];
//     for i in 0..input.len() {
//         let fq: Fq = input[i].into();
//         let tmp = FpVar::<Fq>::new_witness(r1cs_core::ns!(cs, "tmp"), || Ok(fq)).unwrap();
//         res[i] = tmp;
//     }
//     res
// }

// // #[derive(Clone)]
// // pub struct summation {
// //     pub x: Vec<u8>,
// //     pub y: Vec<u8>,
// // }

// // impl ConstraintSynthesizer<Fq> for summation {
// //     fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {

// //         let x_fqvar = generate_input(cs.clone(), self.x.clone());
// //         let y_fq: Fq = self.y[0].into();
// //         let y_fqvar = FpVar::<Fq>::new_witness(r1cs_core::ns!(cs, "tmp"), || Ok(y_fq)).unwrap();

// //         let mut tmp =
// //         FpVar::<Fq>::new_witness(r1cs_core::ns!(cs, "summation gadget"), || {
// //             Ok(Fq::zero())
// //         })
// //         .unwrap();
// //         for i in 0..self.x.len() {
// //             tmp += x_fqvar[i].clone();
// //         }

// //         tmp.enforce_equal(&y_fqvar).unwrap();
// //         Ok(())
// //     }
// // }

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

fn test_native_equality() {
    let rng = &mut test_rng();
    let cs = ConstraintSystem::<Fr>::new_ref();

    // let (input, input_var) = generate_u8_input(cs.clone(), 189, rng);
    // println!("number of constraints for input: {}", cs.num_constraints());

    // let parameters = TestCRH::setup(rng).unwrap();
    // let primitive_result = TestCRH::evaluate(&parameters, input.as_slice()).unwrap();

    // let parameters_var =
    //     <TestCRHGadget as CRHSchemeGadget<TestCRH, Fr>>::ParametersVar::new_witness(
    //         ark_relations::ns!(cs, "parameters_var"),
    //         || Ok(&parameters),
    //     )
    //     .unwrap();
    // println!(
    //     "number of constraints for input + params: {}",
    //     cs.num_constraints()
    // );

    // let result_var = TestCRHGadget::evaluate(&parameters_var, &input_var).unwrap();

    // println!("number of constraints total: {}", cs.num_constraints());

    // assert_eq!(primitive_result, result_var.value().unwrap());
    // assert!(cs.is_satisfied().unwrap());
}

#[derive(Clone)]
pub struct BH {
    pub input: Vec<u8>,
}

impl ConstraintSynthesizer<Fq> for BH {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fq>) -> Result<(), SynthesisError> {

        let (input, input_var) = generate_u8_input(cs.clone(), self.input.clone());
        println!("number of constraints for input: {}", cs.num_constraints());

        Ok(())
    }
}


// // TODO: copy arkwork bowe_hopw code here.

fn main() {
    let cs = ConstraintSystem::<Fr>::new_ref();

    size = 189; // following generate_u8_input()
    let rng = &mut test_rng();
    let mut input = vec![1u8; size];
    rng.fill_bytes(&mut input);

    // Get correct input and output of bowe_hopw here.

    // Get circuit here
    let bh_circuit = BH {
        input: input.clone(),
    };

    // Setup
    println!("start generating random parameters");
    let begin = Instant::now();
    let param =
        generate_random_parameters::<_, _, _>(bh_circuit.clone(), &mut rng)
            .unwrap();
    let end = Instant::now();
        println!("setup time {:?}", end.duration_since(begin));

    let pvk = prepare_verifying_key(&param.vk);
    println!("random parameters generated!\n");
    
    // Prover
    let begin = Instant::now();
    let proof = create_random_proof(bh_circuit, &param, &mut rng).unwrap();
    let end = Instant::now();
    println!("prove time {:?}", end.duration_since(begin));

    // Verifier
    let inputs: Vec<Fq> = vec![];
    let begin = Instant::now();
    assert!(verify_proof(&pvk, &proof, &inputs[..]).unwrap());
    let end = Instant::now();
    println!("verification time {:?}", end.duration_since(begin));

    println!("Run successfully!")
}
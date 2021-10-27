// TODO: 
//    1. Match API version with BH hashing in Arkworks
//    2. Call BH native and circuit function here.
use ark_bls12_381::*;
use ark_ff::{Field, PrimeField, FpParameters, BigInteger};
use ark_std::{One, Zero, UniformRand};

use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::eq::EqGadget;
// use ark_r1cs_std::ed_on_bls12_381::FqVar;

// // use r1cs_core::*;

use ark_groth16::*;
use std::time::Instant;

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


// // TODO: copy arkwork bowe_hopw code here.

fn main() {
    let cs = ConstraintSystem::<Fr>::new_ref();

    // Get correct input and output of bowe_hopw here.
    let mut rng = rand::thread_rng();
    let x: Vec<u8> = vec![1,2,3];
    let y: Vec<u8> = vec![6];

    // // let parameters = TestCRH::setup(rng).unwrap();
    // // let primitive_result = TestCRH::evaluate(&parameters, input.as_slice()).unwrap();

    // // Get circuit here
    // // bh_circuit = generate_bh_circuit(cs, rng, parameters, primitive_result);
    // let full_circuit = summation {
    //     x: x.clone(),
    //     y: y.clone(),
    // };

//     // // Setup
//     // println!("start generating random parameters");
//     // let begin = Instant::now();
//     // let param =
//     //     generate_random_parameters::<algebra::Bls12_381, _, _>(full_circuit.clone(), &mut rng)
//     //         .unwrap();
//     // let end = Instant::now();
//     //     println!("setup time {:?}", end.duration_since(begin));

//     // let pvk = prepare_verifying_key(&param.vk);
//     // println!("random parameters generated!\n");
    
//     // // Prover
//     // let begin = Instant::now();
//     // let proof = create_random_proof(full_circuit, &param, &mut rng).unwrap();
//     // let end = Instant::now();
//     // println!("prove time {:?}", end.duration_since(begin));

//     // // Verifier
//     // let inputs: Vec<Fq> = vec![];
//     // let begin = Instant::now();
//     // assert!(verify_proof(&pvk, &proof, &inputs[..]).unwrap());
//     // let end = Instant::now();
//     // println!("verification time {:?}", end.duration_since(begin));

//     println!("Run successfully!")
}
// Implmenetation for sbox in plaintext and circuit.
// This implementation will be used in poseidon hash.
// Code borrows largely from https://github.com/webb-tools/arkworks-gadgets/tree/master/arkworks-utils/src/poseidon
use ark_plonk::proof_system::{Proof, Prover, ProverKey, Verifier, VerifierKey};
use ark_plonk::circuit::{self, Circuit, PublicInputValue, VerifierData, verify_proof, FeIntoPubInput, GeIntoPubInput};
use ark_plonk::{
	constraint_system::StandardComposer, error::Error, prelude::Variable,
};

use ark_poly_commit::kzg10::KZG10;
use ark_ec::models::TEModelParameters;
use ark_ec::{
    PairingEngine, ProjectiveCurve,
};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::kzg10::{self, Powers, UniversalParams};
use ark_poly_commit::sonic_pc::SonicKZG10;
use ark_poly_commit::PolynomialCommitment;
use ark_serialize::*;
use ark_ff::PrimeField;


use ark_crypto_primitives::crh::poseidon::Poseidon;
use ark_std::{marker::PhantomData, vec::Vec, One, Zero};

use arkworks_plonk_circuits::poseidon::sbox::{PoseidonSbox, SboxConstraints};
use arkworks_plonk_circuits::poseidon::poseidon::{PoseidonParameters, PoseidonParametersVar, hash};
use std::time::Instant;

// Copied from https://github.com/webb-tools/arkworks-gadgets/blob/master/arkworks-plonk-circuits/src/poseidon/poseidon.rs
// The original PoseidonCircuit in the link is private.
#[derive(Debug, Default)]
struct PoseidonCircuit<E: PairingEngine, P: TEModelParameters<BaseField = E::Fr>> {
	pub a: E::Fr,
	pub b: E::Fr,
	pub c: E::Fr,
	pub params: PoseidonParameters<E::Fr>,
	pub _marker: PhantomData<P>,
}


impl<E: PairingEngine, P: TEModelParameters<BaseField = E::Fr>> Circuit<E, P>
	for PoseidonCircuit<E, P>
{
	const CIRCUIT_ID: [u8; 32] = [0xff; 32];

	fn gadget(&mut self, composer: &mut StandardComposer<E, P>) -> Result<(), Error> {
		// ADD INPUTS
		let a = composer.add_input(self.a);
		let b = composer.add_input(self.b);
		let state_zero = composer.add_input(E::Fr::zero());

		let mut round_key_vars = vec![];
		for i in 0..self.params.round_keys.len() {
			let round_key = composer.add_input(self.params.round_keys[i]);
			round_key_vars.push(round_key);
		}

		let mut mds_matrix_vars = vec![];
		for i in 0..self.params.mds_matrix.len() {
			let mut mds_row_vars = vec![];
			for j in 0..self.params.mds_matrix[i].len() {
				let mds_entry = composer.add_input(self.params.mds_matrix[i][j]);
				mds_row_vars.push(mds_entry);
			}
			mds_matrix_vars.push(mds_row_vars);
		}

		let params = PoseidonParametersVar {
			round_keys: round_key_vars,
			mds_matrix: mds_matrix_vars,
			full_rounds: self.params.full_rounds,
			partial_rounds: self.params.partial_rounds,
			width: self.params.width,
			sbox: self.params.sbox,
		};

		let state = vec![state_zero, a, b];
		let computed_hash = hash(state, params, composer)?;

		let add_result = composer.add(
			(E::Fr::one(), computed_hash),
			(E::Fr::one(), composer.zero_var()),
			E::Fr::zero(),
			Some(-self.c),
		);
		composer.assert_equal(add_result, composer.zero_var());

		println!("Composer circuit size: {}", composer.circuit_size());
		Ok(())
	}

	fn padded_circuit_size(&self) -> usize {
		1 << 11
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    
	use ark_bls12_381::{Bls12_381, Fr as Bls12_381Fr};
	use ark_ed_on_bls12_381::{EdwardsParameters as JubjubParameters381, Fq as Fq381};
	use ark_bn254::{Bn254, Fr as Bn254Fr};
	use ark_crypto_primitives::crh::TwoToOneCRH;
	use ark_ed_on_bn254::{EdwardsParameters as JubjubParameters, Fq};
	use ark_ff::{BigInteger, Field};
	use ark_plonk::{
		circuit::{self, FeIntoPubInput},
		prelude::*,
		proof_system::{Prover, Verifier},
	};
	use ark_poly::polynomial::univariate::DensePolynomial;
	use ark_poly_commit::{
		kzg10::{self, UniversalParams, KZG10},
	};
	use ark_std::{test_rng, One};
	use arkworks_utils::utils::common::{setup_params_x5_3};

	type PoseidonCRH3 = arkworks_gadgets::poseidon::CRH<Fq>;
	type PoseidonCRH3_381 = arkworks_gadgets::poseidon::CRH<Fq381>;
	type StandardComposerBn254 =
		ark_plonk::constraint_system::StandardComposer<Bn254, JubjubParameters>;

	#[test]
	fn poseidon_two_to_one_hash_BN254() {
		let curve = arkworks_utils::utils::common::Curve::Bn254;

		let util_params = setup_params_x5_3(curve);
		let params = PoseidonParameters {
			round_keys: util_params.clone().round_keys,
			mds_matrix: util_params.clone().mds_matrix,
			full_rounds: util_params.clone().full_rounds,
			partial_rounds: util_params.clone().partial_rounds,
			sbox: PoseidonSbox::Exponentiation(5),
			width: util_params.clone().width,
		};

		let left_input = Fq::one().into_repr().to_bytes_le();
		let right_input = Fq::one().double().into_repr().to_bytes_le();
		let poseidon_res =
			<PoseidonCRH3 as TwoToOneCRH>::evaluate(&util_params, &left_input, &right_input)
				.unwrap();
		println!("RESULT: {:?}", poseidon_res.to_string());
		let mut circuit = PoseidonCircuit::<Bn254, JubjubParameters> {
			a: Fq::from_le_bytes_mod_order(&left_input),
			b: Fq::from_le_bytes_mod_order(&right_input),
			c: poseidon_res,
			params,
			_marker: std::marker::PhantomData,
		};

		let rng = &mut test_rng();
		let u_params: UniversalParams<Bn254> =
			KZG10::<Bn254, DensePolynomial<Bn254Fr>>::setup(1 << 12, false, rng).unwrap();

		let (pk, vd) = circuit.compile(&u_params).unwrap();

		let begin = Instant::now();		// PROVER
		let proof = {
			let util_params = setup_params_x5_3(curve);
			let params = PoseidonParameters {
				round_keys: util_params.round_keys,
				mds_matrix: util_params.mds_matrix,
				full_rounds: util_params.full_rounds,
				partial_rounds: util_params.partial_rounds,
				sbox: PoseidonSbox::Exponentiation(5),
				width: util_params.width,
			};

			let mut circuit = PoseidonCircuit::<Bn254, JubjubParameters> {
				a: Fq::from_le_bytes_mod_order(&left_input),
				b: Fq::from_le_bytes_mod_order(&right_input),
				c: poseidon_res,
				params,
				_marker: std::marker::PhantomData,
			};
			let begin = Instant::now();		// PROVER
			let tmp = circuit.gen_proof(&u_params, pk, b"Poseidon Test").unwrap();
			let end = Instant::now();
			println!("BN254 prover time {:?}", end.duration_since(begin));
			tmp
		};
		let end = Instant::now();
		println!("BN254 prover time {:?}", end.duration_since(begin));

		// VERIFIER
		let public_inputs: Vec<PublicInputValue<JubjubParameters>> = vec![poseidon_res.into_pi()];

		let VerifierData { key, pi_pos } = vd;

		let begin = Instant::now();		// verifier
		circuit::verify_proof(
			&u_params,
			key,
			&proof,
			&public_inputs,
			&pi_pos,
			b"Poseidon Test",
		)
		.unwrap();
		let end = Instant::now();
        println!("BN254 verifier time {:?}", end.duration_since(begin));
	}

	#[test]
	fn poseidon_two_to_one_hash_BLS381() {
		let curve = arkworks_utils::utils::common::Curve::Bls381;

		let util_params = setup_params_x5_3(curve);
		let params = PoseidonParameters {
			round_keys: util_params.clone().round_keys,
			mds_matrix: util_params.clone().mds_matrix,
			full_rounds: util_params.clone().full_rounds,
			partial_rounds: util_params.clone().partial_rounds,
			sbox: PoseidonSbox::Exponentiation(5),
			width: util_params.clone().width,
		};

		let left_input = Fq381::one().into_repr().to_bytes_le();
		let right_input = Fq381::one().double().into_repr().to_bytes_le();
		let poseidon_res =
			<PoseidonCRH3_381 as TwoToOneCRH>::evaluate(&util_params, &left_input, &right_input)
				.unwrap();
		println!("RESULT: {:?}", poseidon_res.to_string());
		let mut circuit = PoseidonCircuit::<Bls12_381, JubjubParameters381> {
			a: Fq381::from_le_bytes_mod_order(&left_input),
			b: Fq381::from_le_bytes_mod_order(&right_input),
			c: poseidon_res,
			params,
			_marker: std::marker::PhantomData,
		};

		let rng = &mut test_rng();
		let u_params: UniversalParams<Bls12_381> =
			KZG10::<Bls12_381, DensePolynomial<Bls12_381Fr>>::setup(1 << 12, false, rng).unwrap();

		let (pk, vd) = circuit.compile(&u_params).unwrap();

		let proof = {
			let util_params = setup_params_x5_3(curve);
			let params = PoseidonParameters {
				round_keys: util_params.round_keys,
				mds_matrix: util_params.mds_matrix,
				full_rounds: util_params.full_rounds,
				partial_rounds: util_params.partial_rounds,
				sbox: PoseidonSbox::Exponentiation(5),
				width: util_params.width,
			};

			let mut circuit = PoseidonCircuit::<Bls12_381, JubjubParameters381> {
				a: Fq381::from_le_bytes_mod_order(&left_input),
				b: Fq381::from_le_bytes_mod_order(&right_input),
				c: poseidon_res,
				params,
				_marker: std::marker::PhantomData,
			};
			let begin = Instant::now();		// PROVER
			let tmp = circuit.gen_proof(&u_params, pk, b"Poseidon Test").unwrap();
			let end = Instant::now();
			println!("BLS381 prover time {:?}", end.duration_since(begin));
			tmp
		};

		// VERIFIER
		let public_inputs: Vec<PublicInputValue<JubjubParameters381>> = vec![poseidon_res.into_pi()];

		let VerifierData { key, pi_pos } = vd;

		let begin = Instant::now();		// verifier
		circuit::verify_proof(
			&u_params,
			key,
			&proof,
			&public_inputs,
			&pi_pos,
			b"Poseidon Test",
		)
		.unwrap();
		let end = Instant::now();
        println!("BLS381 verifier time {:?}", end.duration_since(begin));
	}
}
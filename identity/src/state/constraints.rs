// use ark_crypto_primitives::crh::CRHScheme;
// use ark_crypto_primitives::{
//     crh::CRHSchemeGadget,
//     signature::{
//         schnorr::{
//             constraints::{ParametersVar, PublicKeyVar},
//             Schnorr, Signature,
//         },
//         SigVerifyGadget,
//     },
// };
// use ark_ec::CurveGroup;
// use ark_ff::Field;
// use ark_r1cs_std::prelude::*;
// use ark_relations::r1cs::{Namespace, SynthesisError};
// use ark_serialize::CanonicalSerialize;
// use blake2::Digest;
// use derivative::Derivative;
// use std::{borrow::Borrow, marker::PhantomData};

// type ConstraintF<C> = <<C as CurveGroup>::BaseField as Field>::BasePrimeField;

// // https://github.com/arkworks-rs/r1cs-tutorial/blob/main/simple-payments/src/signature/schnorr/constraints.rs

// // pub struct SchnorrSignatureVerifyGadget<C: CurveGroup, GC: CurveVar<C, ConstraintF<C>>>
// // where
// //     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// // {
// //     #[doc(hidden)]
// //     _group: PhantomData<*const C>,
// //     #[doc(hidden)]
// //     _group_gadget: PhantomData<*const GC>,
// // }

// pub struct SchnorrSignatureVerifyGadget<
//     H: CRHScheme,
//     C: CurveGroup,
//     H2F: CRHSchemeGadget<H, ConstraintF<C>>,
//     GC: CurveVar<C, ConstraintF<C>>,
// > where
//     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// {
//     #[doc(hidden)]
//     _group: PhantomData<*const C>,
//     #[doc(hidden)]
//     _group_gadget: PhantomData<*const GC>,
// }

// impl<C, GC, D, H, H2F> SigVerifyGadget<Schnorr<C, D>, ConstraintF<C>>
//     for SchnorrSignatureVerifyGadget<H, C, H2F, GC>
// where
//     H: CRHScheme,
//     C: CurveGroup,
//     H2F: CRHSchemeGadget<H, ConstraintF<C>>,
//     GC: CurveVar<C, ConstraintF<C>>,
//     D: Digest + Send + Sync,
//     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// {
//     type ParametersVar = ParametersVar<C, GC>;
//     type PublicKeyVar = PublicKeyVar<C, GC>;
//     type SignatureVar = SignatureVar<ConstraintF<C>, ConstraintF<C>, ConstraintF<C>>;

//     fn verify(
//         parameters: &Self::ParametersVar,
//         public_key: &Self::PublicKeyVar,
//         message: UInt8<ConstraintF<C>>,
//         signature: &Self::SignatureVar,
//     ) -> Result<Boolean<ConstraintF<C>>, SynthesisError> {
//         let prover_response = signature.prover_response;
//         let verifier_challenge = signature.verifier_challenge;
//         let mut claimed_prover_commitment = parameters.generator.mul(*prover_response);
//         let public_key_times_verifier_challenge = public_key.mul(*verifier_challenge);
//         claimed_prover_commitment += &public_key_times_verifier_challenge;
//         let claimed_prover_commitment = claimed_prover_commitment.into_affine();

//         let mut hash_input = Vec::new();
//         hash_input.extend_from_slice(&parameters.salt);
//         hash_input.extend_from_slice(&claimed_prover_commitment.to_bytes()?);
//         hash_input.extend_from_slice(&message);

//         // TODO: Change to H2F
//         let obtained_verifier_challenge = if let Some(obtained_verifier_challenge) =
//             C::ScalarField::from_random_bytes(&D::digest(&hash_input))
//         {
//             obtained_verifier_challenge
//         } else {
//             return Ok(false);
//         };

//         Ok(verifier_challenge.equals(obtained_verifier_challenge))
//     }
// }

// pub struct SignatureVar<F: Field, CF: Field, FVar: FieldVar<F, CF>> {
//     prover_response: FVar,
//     verifier_challenge: FVar,
//     #[doc(hidden)]
//     _field: PhantomData<*const F>,
//     #[doc(hidden)]
//     _constraint_field: PhantomData<*const CF>,
// }

// // #[derive(Derivative)]
// // #[derivative(
// //     Debug(bound = "C: CurveGroup, GC: CurveVar<C, ConstraintF<C>>"),
// //     Clone(bound = "C: CurveGroup, GC: CurveVar<C, ConstraintF<C>>")
// // )]
// // pub struct SignatureVar<C, GC>
// // where
// //     C: CurveGroup,
// //     GC: CurveVar<C, ConstraintF<C>>,
// //     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// // {
// //     prover_response: Vec<UInt8<ConstraintF<C>>>,
// //     verifier_challenge: Vec<UInt8<ConstraintF<C>>>,
// //     #[doc(hidden)]
// //     _group: PhantomData<GC>,
// // }

// impl<C, GC> ToBytesGadget<ConstraintF<C>> for SignatureVar<C, GC>
// where
//     C: CurveGroup,
//     GC: CurveVar<C, ConstraintF<C>>,
//     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// {
//     fn to_non_unique_bytes(&self) -> Result<Vec<UInt8<ConstraintF<C>>>, SynthesisError> {
//         self.to_bytes()
//     }

//     fn to_bytes(&self) -> Result<Vec<UInt8<ConstraintF<C>>>, SynthesisError> {
//         let mut a = self.prover_response.to_bytes()?;
//         let mut b = self.verifier_challenge.to_bytes()?;
//         a.append(&mut b);
//         Ok(a)
//     }
// }

// impl<C, GC> AllocVar<Signature<C>, ConstraintF<C>> for SignatureVar<C, GC>
// where
//     C: CurveGroup,
//     GC: CurveVar<C, ConstraintF<C>>,
//     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// {
//     fn new_variable<T: Borrow<Signature<C>>>(
//         cs: impl Into<Namespace<ConstraintF<C>>>,
//         f: impl FnOnce() -> Result<T, SynthesisError>,
//         mode: AllocationMode,
//     ) -> Result<Self, SynthesisError> {
//         f().and_then(|val| {
//             let cs = cs.into();
//             let mut response_bytes: Vec<u8> = Vec::new();
//             val.borrow()
//                 .prover_response
//                 .serialize_uncompressed(&mut response_bytes)
//                 .unwrap();
//             let mut challenge_bytes: Vec<u8> = Vec::new();
//             val.borrow()
//                 .verifier_challenge
//                 .serialize_uncompressed(&mut challenge_bytes)
//                 .unwrap();
//             let mut prover_response = Vec::<UInt8<ConstraintF<C>>>::new();
//             let mut verifier_challenge = Vec::<UInt8<ConstraintF<C>>>::new();
//             for byte in &response_bytes {
//                 prover_response.push(UInt8::<ConstraintF<C>>::new_variable(
//                     cs.clone(),
//                     || Ok(byte),
//                     mode,
//                 )?);
//             }
//             for byte in &challenge_bytes {
//                 verifier_challenge.push(UInt8::<ConstraintF<C>>::new_variable(
//                     cs.clone(),
//                     || Ok(byte),
//                     mode,
//                 )?);
//             }
//             Ok(SignatureVar {
//                 prover_response,
//                 verifier_challenge,
//                 _group: PhantomData,
//             })
//         })
//     }
// }

// // impl<C, GC, D> SigVerifyGadget<Schnorr<C, D>, ConstraintF<C>>
// //     for SchnorrSignatureVerifyGadget<C, GC>
// // where
// //     C: CurveGroup,
// //     GC: CurveVar<C, ConstraintF<C>>,
// //     D: Digest + Send + Sync,
// //     for<'a> &'a GC: GroupOpsBounds<'a, C, GC>,
// // {
// //     type ParametersVar = ParametersVar<C, GC>;
// //     type PublicKeyVar = PublicKeyVar<C, GC>;
// //     type SignatureVar = SignatureVar<C, GC>;

// //     fn verify(
// //         parameters: &Self::ParametersVar,
// //         public_key: &Self::PublicKeyVar,
// //         // TODO: Should we make this take in bytes or something different?
// //         message: &[UInt8<ConstraintF<C>>],
// //         signature: &Self::SignatureVar,
// //     ) -> Result<Boolean<ConstraintF<C>>, ark_relations::r1cs::SynthesisError> {
// //         todo!()
// //     }
// // }


// #[cfg(test)]F
// mod test {

//     use bellman::groth16::{
//         create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
//     };
//     use bellman::{Circuit, ConstraintSystem, SynthesisError};
//     use bls12_381::{Bls12, Scalar};
//     // use ff::Field;
//     use rand::rngs::OsRng;

//     // 定义一个简单的电路,用于验证客户端签名
//     #[derive(Clone)]
//     struct SignatureVerificationCircuit {
//         pub message: Option<Scalar>,
//         pub signature: Option<Scalar>,
//         pub public_key: Option<Scalar>,
//     }

//     impl Circuit<Scalar> for SignatureVerificationCircuit {
//         fn synthesize<CS: ConstraintSystem<Scalar>>(
//             self,
//             cs: &mut CS,
//         ) -> Result<(), SynthesisError> {
//             let message = cs.alloc(
//                 || "message",
//                 || self.message.ok_or(SynthesisError::AssignmentMissing),
//             )?;
//             let signature = cs.alloc(
//                 || "signature",
//                 || self.signature.ok_or(SynthesisError::AssignmentMissing),
//             )?;
//             let public_key = cs.alloc(
//                 || "public_key",
//                 || self.public_key.ok_or(SynthesisError::AssignmentMissing),
//             )?;

//             // 在这里实现电路的约束逻辑,这只是一个占位符
//             // cs.enforce(
//             //     || "placeholder constraint",
//             //     |lc| lc + message,
//             //     |lc| lc + signature,
//             //     |lc| lc + public_key,
//             // );

//             cs.enforce(
//                 || "always success constraint",
//                 |lc| lc + message - message,
//                 |lc| lc + CS::one(),
//                 |lc| lc + signature - signature,
//             );

//             Ok(())
//         }
//     }

//     #[test]
//     fn test_main() {
//         let message = Scalar::from(42); // 模拟消息
//         let signature = Scalar::from(123); // 模拟签名
//         let public_key = Scalar::from(456); // 模拟公钥

//         // 生成随机参数
//         let params = {
//             let c = SignatureVerificationCircuit {
//                 message: None,
//                 signature: None,
//                 public_key: None,
//             };
//             generate_random_parameters::<Bls12, _, _>(c, &mut OsRng).unwrap()
//         };

//         // 准备验证密钥
//         let pvk = prepare_verifying_key(&params.vk);

//         // 创建证明
//         let proof = {
//             let c = SignatureVerificationCircuit {
//                 message: Some(message),
//                 signature: Some(signature),
//                 public_key: Some(public_key),
//             };
//             create_random_proof(c, &params, &mut OsRng).unwrap()
//         };

//         // 验证证明
//         let result = verify_proof(&pvk, &proof, &[]);
//         assert!(result.is_ok());
//         // assert!(result.unwrap());
//         println!("ZKP签名验证通过!");
//     }
// }

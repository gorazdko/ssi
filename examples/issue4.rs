// To generate text fixture:
// cargo run --example issue > examples/vc.jsonld

#[async_std::main]
async fn main() {
    //let key_str = include_str!("../tests/ed25519-2020-10-18.json");
    let key_str = include_str!("../tests/ed25519-2020-10-18_gorazd.json");
    let key: ssi::jwk::JWK = serde_json::from_str(key_str).unwrap();
    let vc = serde_json::json!({
    "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "http://example.edu/credentials/1872",
    "type": ["VerifiableCredential"],
    "issuer": "did:onion:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid",
    "issuanceDate": "2010-01-01T19:23:24Z",
    "credentialSubject": {
        "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
        "alumniOf": "<span>Example University</span>"
    }
    });
    let mut vc: ssi::vc::Credential = serde_json::from_value(vc).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();
    let verification_method = key.to_verification_method().unwrap();
    proof_options.verification_method = Some(verification_method);
    let proof = vc.generate_proof(&key, &proof_options).await.unwrap();
    vc.add_proof(proof);
    let result = vc.verify(None).await;
    if result.errors.len() > 0 {
        panic!("verify failed: {:#?}", result);
    }
    let stdout_writer = std::io::BufWriter::new(std::io::stdout());
    serde_json::to_writer_pretty(stdout_writer, &vc).unwrap();
}

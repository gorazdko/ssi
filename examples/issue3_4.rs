// To generate text fixture:
// cargo run --example issue > examples/vc.jsonld

#[async_std::main]
async fn main() {
    //let key_str = include_str!("../tests/ed25519-2020-10-18.json");
    let key_str = include_str!("../tests/ed25519-2020-10-18_gorazd.json");
    let key: ssi::jwk::JWK = serde_json::from_str(key_str).unwrap();

    //println!("key:did: {:?}", key.to_did().unwrap());

    let vc = serde_json::json!({
    "@context": [
        "https://www.w3.org/2018/credentials/v1", {
            "programmingLanguage": "https://schema.org/programmingLanguage",
            "SoftwareSourceCode": "https://schema.org/SoftwareSourceCode",
            "codeRepository": "https://schema.org/codeRepository",
            "version": "https://schema.org/version",
            "creator": "https://schema.org/creator"
        }
    ],
    "id": "urn:uuid:".to_string() + &uuid::Uuid::new_v4().to_string(),
    "issuer": "did:onion:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid",
    "type": ["VerifiableCredential", "SoftwareSourceCode"],
    "issuanceDate": "2021-01-10T19:23:24Z",
    "credentialSubject": {
        "id": "urn:uuid:".to_string() + &uuid::Uuid::new_v4().to_string(),
        "codeRepository": "https://github.com/BlockchainCommons/bc-seedtool-cli.git",
        "version": "6b5b64afd16db4b075bfe68d06f6d58f3189f13a",
        "programmingLanguage": "C++",
        "creator": {
            "id": "did:onion:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid",
            "type": "Person|Organization"
        }
    }
    });
    let mut vc: ssi::vc::Credential = serde_json::from_value(vc).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();
    let verification_method = "did:onion:fscst5exmlmr262byztwz4kzhggjlzumvc2ndvgytzoucr2tkgxf7mid#iFPG3dtLcg-0jI4CVa9b94g06KadgrpM8rC9EMI94nA".to_string(); //Some(verification_method);
    proof_options.verification_method = Some(verification_method);
    let proof = vc.generate_proof(&key, &proof_options).await.unwrap();
    vc.add_proof(proof);

    // comment these two lines:
    let stdout_writer = std::io::BufWriter::new(std::io::stdout());
    serde_json::to_writer_pretty(stdout_writer, &vc).unwrap();

    /*  // uncomment this
        let result = vc.verify(None).await;
        if result.errors.len() > 0 {
            panic!("verify failed: {:#?}", result);
        }
        let stdout_writer = std::io::BufWriter::new(std::io::stdout());
        serde_json::to_writer_pretty(stdout_writer, &vc).unwrap();
    */
}

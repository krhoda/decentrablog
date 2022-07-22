use crate::{
    schema::schema_type::{SchemaError, SchemaType},
    signer::signer::{SignerError, SignerType, DID},
    witness::{
        signer_type::SignerTypes,
        witness::{Generator, Proof, Statement, WitnessError},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, Serialize)]
pub struct Opts {
    pub title: String,
    pub body: String,
    pub key_type: DID,
}

impl Statement for Opts {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        Ok(format!("{}{}{}", self.title, self.delimitor(), self.body))
    }

    fn delimitor(&self) -> String {
        ":::".to_string()
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Claim {
    pub signature: String,
    pub statement_opts: Opts,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        self.statement_opts.signer_type()
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        self.statement_opts.generate_statement()
    }

    fn delimitor(&self) -> String {
        self.statement_opts.delimitor()
    }
}

// TODO: Can we just derive this?
impl Proof for Claim {}

impl SchemaType for Claim {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS MORE ACCURATE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "BasicPostCredential": "https://example.com/BasicPostCredential",
                "BasicPost": {
                    "@id": "https://schema.org/BasicPost",
                    "@context": {
                        "title": "https://schema.org/name",
                        "body": "https://schema.org/articleBody",
                    }
                }
            },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicPost".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        let signer_type = SignerTypes::new(&self.statement_opts.key_type)?;
        let signer_did = signer_type
            .did_id()
            .map_err(|e| SchemaError::BadSubject(e.to_string()))?;

        Ok(json!({
            "id": signer_did,
            "type": ["BasicPost"],
            "title": self.statement_opts.title,
            "body": self.statement_opts.body,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }
}

pub struct ClaimGenerator {}

#[async_trait(?Send)]
impl Generator<Claim, Claim> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        Ok(format!(
            "{}{}{}",
            &proof.statement_opts.generate_statement()?,
            &proof.statement_opts.delimitor(),
            &proof.signature
        ))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        _statement: &str,
        _signature: &str,
    ) -> Result<Claim, WitnessError> {
        Ok(proof.clone())
    }
}

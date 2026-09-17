use crate::key_manager::{ECJWK, JWK, JWKS, RSAJWK};
use base64::prelude::*;
use openssl::{
    bn::{BigNum, BigNumContext},
    ec::{EcGroup, EcKey},
    nid::Nid,
    pkey::PKey,
    rand::rand_bytes,
    rsa::Rsa,
};
use std::{error, fs, io::BufWriter, path::Path};
#[derive(Debug, Clone)]
pub struct KeyWorker {
    base_dir: &'static str,
    alg_supported: Vec<&'static str>,
}

impl KeyWorker {
    pub fn new(base_dir: &'static str, alg_supported: Vec<&'static str>) -> Self {
        Self {
            base_dir,
            alg_supported,
        }
    }

    pub fn generate(&self) -> Result<(), Box<dyn error::Error>> {
        fs::create_dir_all(format!("{}/private", self.base_dir))?;
        fs::create_dir_all(format!("{}/public", self.base_dir))?;
        for alg in self.alg_supported.clone().into_iter() {
            let priv_key_path = format!("{}/private/{}_priv.pem", self.base_dir, alg);
            let pub_key_path = format!("{}/public/{}_pub.pem", self.base_dir, alg);
            match alg {
                "RS256" | "RSA-OAEP-256" => {
                    let pkey = if Path::new(priv_key_path.as_str()).exists() {
                        tracing::info!("{} private key already exists, loading it", alg);
                        let priv_pem = fs::read(priv_key_path.as_str())?;
                        PKey::private_key_from_pem(&priv_pem)?
                    } else {
                        tracing::info!("{} generating a new keypair", alg);
                        let rsa = Rsa::generate(2048)?;
                        let pkey = PKey::from_rsa(rsa)?;
                        tracing::info!("{} generating private key", alg);
                        fs::write(priv_key_path.as_str(), pkey.private_key_to_pem_pkcs8()?)?;
                        pkey
                    };
                    if !Path::new(pub_key_path.as_str()).exists() {
                        tracing::info!("{} generating public key", alg);
                        fs::write(pub_key_path.as_str(), pkey.public_key_to_pem()?)?;
                    }
                }

                "HS256" | "A128KW" => {
                    let secret_path = priv_key_path.replace("priv.pem", "secret.key");
                    if !Path::new(secret_path.as_str()).exists() {
                        tracing::info!("{} secret generating...", alg);

                        let mut secret = vec![0u8; if alg == "A128KW" { 16 } else { 32 }];
                        rand_bytes(&mut secret)?;
                        fs::write(secret_path.as_str(), secret)?;
                    } else {
                        tracing::info!("{} secret already exists, skipping generation", alg);
                    }
                }

                "ES256" => {
                    let pkey = if Path::new(priv_key_path.as_str()).exists() {
                        tracing::info!("{} private key already exists, loading it", alg);
                        let priv_pem = fs::read(priv_key_path.as_str())?;
                        PKey::private_key_from_pem(&priv_pem)?
                    } else {
                        tracing::info!("{} generating a new keypair", alg);
                        let ec_key = EcKey::generate(
                            &EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?.as_ref(),
                        )?;
                        let pkey = PKey::from_ec_key(ec_key)?;
                        tracing::info!("{} generating private key", alg);
                        fs::write(priv_key_path.as_str(), pkey.private_key_to_pem_pkcs8()?)?;
                        pkey
                    };

                    if !Path::new(pub_key_path.as_str()).exists() {
                        tracing::info!("{} generating public key", alg);
                        fs::write(pub_key_path.as_str(), pkey.public_key_to_pem()?)?;
                    }
                }
                _ => {
                    tracing::warn!("unsupported key algorithm: {}", alg);
                    return Err(format!("unsupported key algorithm: {}", alg).into());
                }
            }
        }

        let _ = self.create_jwks();
        Ok(())
    }

    pub fn create_jwks(&self) -> Result<(), Box<dyn error::Error>> {
        let public_key_sets = fs::read_dir(format!("{}/public/", self.base_dir))?
            .map(|entry| -> Option<(&'static str, Vec<u8>)> {
                let entry = entry.ok()?;
                let path = entry.path();
                let filename = Box::leak(
                    path.file_name()?
                        .to_string_lossy()
                        .into_owned()
                        .into_boxed_str(),
                );
                let alg = filename.split('_').next()?;

                Some((alg, fs::read(path).ok()?))
            })
            .flatten()
            .collect::<Vec<(&str, Vec<u8>)>>();

        let keys = public_key_sets
            .iter()
            .map(
                |&(alg, ref pub_pem)| -> Result<JWK, Box<dyn std::error::Error>> {
                    match alg {
                        "RS256" => {
                            let pkey = Rsa::public_key_from_pem(&pub_pem)?;
                            Ok(JWK::RSA(RSAJWK {
                                kty: "RSA".to_string(),
                                r#use: "sig".to_string(),
                                kid: format!("{}-<kid>", alg),
                                alg: alg.to_string(),
                                n: BASE64_URL_SAFE_NO_PAD.encode(pkey.n().to_vec()),
                                e: BASE64_URL_SAFE_NO_PAD.encode(pkey.e().to_vec()),
                            }))
                        }
                        "RSA-OAEP-256" => {
                            let pkey = Rsa::public_key_from_pem(&pub_pem)?;
                            Ok(JWK::RSA(RSAJWK {
                                kty: "RSA".to_string(),
                                r#use: "enc".to_string(),
                                kid: format!("{}-<kid>", alg),
                                alg: alg.to_string(),
                                n: BASE64_URL_SAFE_NO_PAD.encode(pkey.n().to_vec()),
                                e: BASE64_URL_SAFE_NO_PAD.encode(pkey.e().to_vec()),
                            }))
                        }
                        "ES256" => {
                            let pkey = EcKey::public_key_from_pem(&pub_pem)?;
                            let mut ctx = BigNumContext::new()?;
                            let mut x = BigNum::new()?;
                            let mut y = BigNum::new()?;
                            pkey.public_key().affine_coordinates(
                                pkey.group(),
                                &mut x,
                                &mut y,
                                &mut ctx,
                            )?;
                            Ok(JWK::EC(ECJWK {
                                kty: "EC".to_string(),
                                r#use: "sig".to_string(),
                                kid: format!("{}-<kid>", alg),
                                alg: alg.to_string(),
                                crv: "P-256".to_string(),
                                x: BASE64_URL_SAFE_NO_PAD.encode(x.to_vec()),
                                y: BASE64_URL_SAFE_NO_PAD.encode(y.to_vec()),
                            }))
                        }
                        _ => {
                            tracing::warn!("unsupported key algorithm: {}", alg);
                            return Err(format!("unsupported key algorithm: {}", alg).into());
                        }
                    }
                },
            )
            .flatten()
            .collect::<Vec<JWK>>();

        let file = fs::File::create(format!("{}/jwks.json", self.base_dir))?;
        let writer = BufWriter::new(file);

        let _ = serde_json::to_writer_pretty(writer, &JWKS { keys });

        Ok(())
    }
}

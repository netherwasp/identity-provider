use openssl::{
    ec::{EcGroup, EcKey},
    nid::Nid,
    pkey::PKey,
    rand::rand_bytes,
    rsa::Rsa,
};
use std::{error, fs, path::Path};

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
        for alg in self.alg_supported.clone().into_iter() {
            key_generator(self.base_dir, alg)?;
        }
        Ok(())
    }
}

pub fn key_generator(base_dir: &str, alg: &str) -> Result<(), Box<dyn error::Error>> {
    fs::create_dir_all(format!("{base_dir}/private"))?;
    fs::create_dir_all(format!("{base_dir}/public"))?;
    let priv_key_path = format!("{base_dir}/private/{alg}_priv.pem");
    let pub_key_path = format!("{base_dir}/public/{alg}_pub.pem");
    match alg {
        "RS256" | "RSA-OAEP-256" => {
            let pkey = if Path::new(priv_key_path.as_str()).exists() {
                tracing::info!("{alg} private key already exists, loading it");
                let priv_pem = fs::read(priv_key_path.as_str())?;
                PKey::private_key_from_pem(&priv_pem)?
            } else {
                tracing::info!("{alg} generating a new keypair");
                let rsa = Rsa::generate(2048)?;
                let pkey = PKey::from_rsa(rsa)?;
                tracing::info!("{alg} generating private key");
                fs::write(priv_key_path.as_str(), pkey.private_key_to_pem_pkcs8()?)?;
                pkey
            };
            if !Path::new(pub_key_path.as_str()).exists() {
                tracing::info!("{alg} generating public key");
                fs::write(pub_key_path.as_str(), pkey.public_key_to_pem()?)?;
            }
        }

        "HS256" | "A128KW" => {
            let secret_path = priv_key_path.replace("priv.pem", "secret.key");
            if !Path::new(secret_path.as_str()).exists() {
                tracing::info!("{alg} secret generating...");

                let mut secret = vec![0u8; if alg == "A128KW" { 16 } else { 32 }];
                rand_bytes(&mut secret)?;
                fs::write(secret_path.as_str(), secret)?;
            } else {
                tracing::info!("{alg} secret already exists, skipping generation");
            }
        }

        "ES256" => {
            let pkey = if Path::new(priv_key_path.as_str()).exists() {
                tracing::info!("{alg} private key already exists, loading it");
                let priv_pem = fs::read(priv_key_path.as_str())?;
                PKey::private_key_from_pem(&priv_pem)?
            } else {
                tracing::info!("{alg} generating a new keypair");
                let ec_key =
                    EcKey::generate(&EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?.as_ref())?;
                let pkey = PKey::from_ec_key(ec_key)?;
                tracing::info!("{alg} generating private key");
                fs::write(priv_key_path.as_str(), pkey.private_key_to_pem_pkcs8()?)?;
                pkey
            };

            if !Path::new(pub_key_path.as_str()).exists() {
                tracing::info!("{alg} generating public key");
                fs::write(pub_key_path.as_str(), pkey.public_key_to_pem()?)?;
            }
        }
        _ => {
            tracing::warn!("unsupported key algorithm: {}", alg);
        }
    }
    Ok(())
}

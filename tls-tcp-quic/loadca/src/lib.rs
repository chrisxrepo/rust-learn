use rustls::{
    RootCertStore,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use std::path::Path;

pub fn load_certs(filename: &Path) -> Vec<CertificateDer<'static>> {
    CertificateDer::pem_file_iter(filename)
        .expect("cannot open certificate file")
        .map(|result| result.unwrap())
        .collect()
}

pub fn load_private_key(filename: &Path) -> PrivateKeyDer<'static> {
    PrivateKeyDer::from_pem_file(filename).expect("cannot read private key file")
}

pub fn load_root_ca(filename: &Path) -> RootCertStore {
    let certs = crate::load_certs(filename);

    let mut root_store = RootCertStore::empty();
    for cert in certs {
        root_store.add(cert).unwrap()
    }

    root_store
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let certs = load_certs(Path::new("ca/cert.pem"));
        println!("{:?}", certs);

        println!("-----------------------------------");
        let private_key = load_private_key(Path::new("ca/key.pem"));
        println!("{:?}", private_key);

        let ca = load_root_ca(Path::new("ca/cert.pem"));
        println!("{:?}", ca)
    }
}

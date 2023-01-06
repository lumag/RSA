#![feature(test)]

extern crate test;

use base64ct::{Base64, Encoding};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rsa::signature::RandomizedSigner;
use rsa::{PaddingScheme, RsaPrivateKey};
use sha2::{Digest, Sha256};
use test::Bencher;

const DECRYPT_VAL: &'static str =
    "XW4qfrpQDarEMBfPyIYE9UvuOFkbBi0tiGYbIOJPLMNe/LWuPD0BQ7ceqlOlPPcKLinYz0DlnqW3It/V7ae59zw9afA3YIWdq0Ut2BnYL+aJixnqaP+PjsQNcHg6axCF11iNQ4jpXrZDiQcI+q9EEzZDTMsiMxtjfgBQUd8LHT87YoQXDWaFPCVpliACMc8aUk442kH1tc4jEuXwjEjFErvAM/J7VizCdU/dnKrlq2mBDzvZ6hxY9TYHFB/zY6DZPJAgEMUxYWCR9xPJ7X256DV1Kt0Ht33DWoFcgh/pPLM1q9pK0HVxCdclXfZOeCqlrLgZ5Gxv5DM4BtV7Z4m85w==";

fn get_key() -> RsaPrivateKey {
    RsaPrivateKey::from_components(
        BigUint::from_str_radix("14314132931241006650998084889274020608918049032671858325988396851334124245188214251956198731333464217832226406088020736932173064754214329009979944037640912127943488972644697423190955557435910767690712778463524983667852819010259499695177313115447116110358524558307947613422897787329221478860907963827160223559690523660574329011927531289655711860504630573766609239332569210831325633840174683944553667352219670930408593321661375473885147973879086994006440025257225431977751512374815915392249179976902953721486040787792801849818254465486633791826766873076617116727073077821584676715609985777563958286637185868165868520557", 10).unwrap(),
        BigUint::from_u32(3).unwrap(),
        BigUint::from_str_radix("9542755287494004433998723259516013739278699355114572217325597900889416163458809501304132487555642811888150937392013824621448709836142886006653296025093941418628992648429798282127303704957273845127141852309016655778568546006839666463451542076964744073572349705538631742281931858219480985907271975884773482372966847639853897890615456605598071088189838676728836833012254065983259638538107719766738032720239892094196108713378822882383694456030043492571063441943847195939549773271694647657549658603365629458610273821292232646334717612674519997533901052790334279661754176490593041941863932308687197618671528035670452762731", 10).unwrap(),
        vec![
            BigUint::from_str_radix("130903255182996722426771613606077755295583329135067340152947172868415809027537376306193179624298874215608270802054347609836776473930072411958753044562214537013874103802006369634761074377213995983876788718033850153719421695468704276694983032644416930879093914927146648402139231293035971427838068945045019075433",10).unwrap(),
            BigUint::from_str_radix("109348945610485453577574767652527472924289229538286649661240938988020367005475727988253438647560958573506159449538793540472829815903949343191091817779240101054552748665267574271163617694640513549693841337820602726596756351006149518830932261246698766355347898158548465400674856021497190430791824869615170301029", 10).unwrap()
        ],
    ).unwrap()
}

#[bench]
fn bench_rsa_2048_pkcsv1_decrypt(b: &mut Bencher) {
    let priv_key = get_key();
    let x = Base64::decode_vec(DECRYPT_VAL).unwrap();

    b.iter(|| {
        let res = priv_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &x)
            .unwrap();
        test::black_box(res);
    });
}

#[bench]
fn bench_rsa_2048_pkcsv1_sign_blinded(b: &mut Bencher) {
    let priv_key = get_key();
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new_with_prefix(priv_key);
    let digest = Sha256::digest(b"testing").to_vec();
    let mut rng = ChaCha8Rng::from_seed([42; 32]);

    b.iter(|| {
        let res = signing_key.sign_with_rng(&mut rng, &digest);
        test::black_box(res);
    });
}

#[bench]
fn bench_rsa_2048_pss_sign_blinded(b: &mut Bencher) {
    let priv_key = get_key();
    let signing_key = rsa::pss::SigningKey::<Sha256>::new(priv_key);
    let digest = Sha256::digest(b"testing").to_vec();
    let mut rng = ChaCha8Rng::from_seed([42; 32]);

    b.iter(|| {
        let res = signing_key.sign_with_rng(&mut rng, &digest);
        test::black_box(res);
    });
}

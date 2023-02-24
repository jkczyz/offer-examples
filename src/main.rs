use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
use lightning::chain::keysinterface::EntropySource;
use lightning::offers::offer::OfferBuilder;
use lightning::onion_message::BlindedPath;

fn main() {
	// Zero-amount, empty description

	let offer = OfferBuilder::new("".to_string(), pubkey(42)).build().unwrap();
	println!("Zero-amount offer with empty description:\n{}\n", offer);

	let offer = OfferBuilder::new("".to_string(), pubkey(42))
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.build().unwrap();
	println!("Zero-amount offer with empty description and one blinded path:\n{}\n", offer);

	let offer = OfferBuilder::new("".to_string(), pubkey(42))
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.path(blinded_path(&vec![pubkey(45), pubkey(43)]))
		.build().unwrap();
	println!("Zero-amount offer with empty description and two blinded paths:\n{}\n", offer);

	// Zero-amount, description

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.build().unwrap();
	println!("Zero-amount offer with description:\n{}\n", offer);

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.build().unwrap();
	println!("Zero-amount offer with description and one blinded path:\n{}\n", offer);

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.path(blinded_path(&vec![pubkey(45), pubkey(43)]))
		.build().unwrap();
	println!("Zero-amount offer with description and two blinded paths:\n{}\n", offer);

	// Amount, empty description

	let offer = OfferBuilder::new("".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.build().unwrap();
	println!("Offer with empty description:\n{}\n", offer);

	let offer = OfferBuilder::new("".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.build().unwrap();
	println!("Offer with empty description and one blinded path:\n{}\n", offer);

	let offer = OfferBuilder::new("".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.path(blinded_path(&vec![pubkey(45), pubkey(43)]))
		.build().unwrap();
	println!("Offer with empty description and two blinded paths:\n{}\n", offer);

	// Amount, description

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.build().unwrap();
	println!("Offer with description:\n{}\n", offer);

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.build().unwrap();
	println!("Offer with description and one blinded path:\n{}\n", offer);

	let offer = OfferBuilder::new("Pay Esmeralda Thompson".to_string(), pubkey(42))
		.amount_msats(2_000_000)
		.path(blinded_path(&vec![pubkey(44), pubkey(43)]))
		.path(blinded_path(&vec![pubkey(45), pubkey(43)]))
		.build().unwrap();
	println!("Offer with description and two blinded paths:\n{}\n", offer);
}

struct Randomness;

impl EntropySource for Randomness {
	fn get_secure_random_bytes(&self) -> [u8; 32] {
		rand::random()
	}
}

fn pubkey(byte: u8) -> PublicKey {
	let secp_ctx = Secp256k1::new();
	PublicKey::from_secret_key(&secp_ctx, &privkey(byte))
}

fn privkey(byte: u8) -> SecretKey {
	SecretKey::from_slice(&[byte; 32]).unwrap()
}

fn blinded_path(hops: &[PublicKey]) -> BlindedPath {
	let secp_ctx = Secp256k1::new();
	let randomness = Randomness {};
	BlindedPath::new(hops, &randomness, &secp_ctx).unwrap()
}

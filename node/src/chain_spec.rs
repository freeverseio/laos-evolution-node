use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GrandpaConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// "Names" of the authorities accounts at local testnet.
const LOCAL_AUTHORITIES_ACCOUNTS: [&str; 5] = ["Alice", "Bob", "Charlie", "Dave", "Eve"];
/// "Names" of all possible authorities accounts.
const ALL_AUTHORITIES_ACCOUNTS: [&str; 5] = LOCAL_AUTHORITIES_ACCOUNTS;
/// "Name" of the account, which owns the with-Rococo messages pallet.
const ROCOCO_MESSAGES_PALLET_OWNER: &str = "Rococo.MessagesOwner";
/// "Name" of the account, which owns the with-OwnershipParachain messages pallet.
const OWNERSHIP_PARACHAIN_MESSAGES_PALLET_OWNER: &str = "OwnershipParachain.MessagesOwner";

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// We're using the same set of endowed accounts on all Millau chains (dev/local) to make
/// sure that all accounts, required for bridge to be functional (e.g. relayers fund account,
/// accounts used by relayers in our test deployments, accounts used for demonstration
/// purposes), are all available on these chains.
fn endowed_accounts() -> Vec<AccountId> {
	let all_authorities = ALL_AUTHORITIES_ACCOUNTS.iter().flat_map(|x| {
		[
			get_account_id_from_seed::<sr25519::Public>(x),
			get_account_id_from_seed::<sr25519::Public>(&format!("{x}//stash")),
		]
	});

	vec![
		// Regular (unused) accounts
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		// Accounts, used by Rococo<>Evochain bridge
		get_account_id_from_seed::<sr25519::Public>(ROCOCO_MESSAGES_PALLET_OWNER),
		get_account_id_from_seed::<sr25519::Public>("Rococo.HeadersAndMessagesRelay"),
		get_account_id_from_seed::<sr25519::Public>("Rococo.OutboundMessagesRelay.Lane00000001"),
		get_account_id_from_seed::<sr25519::Public>("Rococo.InboundMessagesRelay.Lane00000001"),
		get_account_id_from_seed::<sr25519::Public>("Rococo.MessagesSender"),
		// Accounts, used by OwnershipParachain<>Evochain bridge
		get_account_id_from_seed::<sr25519::Public>(OWNERSHIP_PARACHAIN_MESSAGES_PALLET_OWNER),
		get_account_id_from_seed::<sr25519::Public>("OwnershipParachain.HeadersAndMessagesRelay1"),
		get_account_id_from_seed::<sr25519::Public>("OwnershipParachain.HeadersAndMessagesRelay2"),
		get_account_id_from_seed::<sr25519::Public>("OwnershipParachain.RococoHeadersRelay1"),
		get_account_id_from_seed::<sr25519::Public>("OwnershipParachain.RococoHeadersRelay2"),
		get_account_id_from_seed::<sr25519::Public>("OwnershipParachain.MessagesSender"),
	]
	.into_iter()
	.chain(all_authorities)
	.collect()
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

type AccountPublic = <Signature as Verify>::Signer;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				endowed_accounts(),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				endowed_accounts(),
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		..Default::default()
	}
}

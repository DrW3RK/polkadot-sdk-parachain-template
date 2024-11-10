use parachain_template_runtime as runtime;
use parachain_template_runtime::{EXISTENTIAL_DEPOSIT, AccountId, AuraId};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{bytes::from_hex, sr25519};

use cumulus_primitives_core::ParaId;
use sp_runtime::traits::{IdentifyAccount};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	#[serde(alias = "relayChain", alias = "RelayChain")]
	pub relay_chain: String,
	/// The id of the Parachain.
	#[serde(alias = "paraId", alias = "ParaId")]
	pub para_id: u32,
}


impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

// Sudo privileges
pub const ROOT_ACCOUNT: &str = "0xf8cb76d7f3bcfe13fa74ec9582ee96fd4c559906f3da0b65fd24f7eb42632246";

// Collator accounts that produce blocks and earn rewards. Typically, private key is in cold storage

pub const COLLATOR1: &str = "0xc4f2fbd1c30d8b84af3a5877afb24108e3c3050758477a1a5af2cde7efb2e444";
pub const COLLATOR2: &str = "0x6ad577d8b96b340abe1a6f86837d41c0040fe7e9b3f00fcd926cf0b4fe388c2f";

// The private key of these session keys needs to be inserted into the collator node for it to start
// producing blocks.

pub const SESSION1: &str = "0x64c0ac9ed7dbb5e818898610ea14277fa9b7baccc86e947df940a984a70bfe7e";
pub const SESSION2: &str = "0xa01476431a8d51d2826284c5dfcf32837f581d44ad1fc0bf55e4330e34b2020b";


pub fn pub_to_account_id(pubkey: &str) -> AccountId {
    let pubkey = sr25519::Public::from_raw(
        from_hex(pubkey)
            .expect("Unable to parse hex")
            .try_into()
            .expect("Unable to parse public key"),
    );
    //dbg!(pubkey.clone().into_account().to_string());
    pubkey.into_account().into()
}

pub fn pub_to_collator_key(pubkey: &str) -> AuraId {
    let pubkey = sr25519::Public::from_raw(
        from_hex(pubkey)
            .expect("Unable to parse hex")
            .try_into()
            .expect("Unable to parse public key"),
    );

    //dbg!(pubkey);

    AuraId::from(pubkey)
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we
/// have just one key).
pub fn template_session_keys(keys: AuraId) -> parachain_template_runtime::SessionKeys {
    parachain_template_runtime::SessionKeys { aura: keys }
}


pub fn live_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "SUB0".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        parachain_template_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "paseo".into(),
            // You MUST set this to the correct network!
            para_id: 4540,
        },
    )
    .with_name("Sub0 Reset")
    .with_id("live")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(livenet_genesis(
        // initial collators.
        vec![
            (pub_to_account_id(COLLATOR1), pub_to_collator_key(SESSION1)),
            (pub_to_account_id(COLLATOR2), pub_to_collator_key(SESSION2)),
        ],
        vec![
            pub_to_account_id(COLLATOR1),
            pub_to_account_id(COLLATOR2),
            pub_to_account_id(ROOT_ACCOUNT),
        ],
        pub_to_account_id(ROOT_ACCOUNT),
        4540.into(),
    ))
    .with_protocol_id("sub0-reset-live")
    .with_properties(properties)
    .build()
}


pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(),
			// You MUST set this to the correct network!
			para_id: 1000,
		},
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
	.build()
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	#[allow(deprecated)]
	ChainSpec::builder(
		runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(),
			// You MUST set this to the correct network!
			para_id: 1000,
		},
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(sc_chain_spec::LOCAL_TESTNET_RUNTIME_PRESET)
	.with_protocol_id("template-local")
	.with_properties(properties)
	.build()
}


fn livenet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
    id: ParaId,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "collatorSelection": {
            "invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
            "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
        },
        "session": {
            "keys": invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(root) }
    })
}
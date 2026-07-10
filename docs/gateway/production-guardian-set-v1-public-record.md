# Production Guardian Set V1 Public Record

Status: SOURCE_CONFIG_BOUND_PUBLIC_KEYS_ONLY_NO_ACTIVATION

Package:

- production-guardian-set-v1-public-record-and-source-change

Approval:

- APPROVE_PRODUCTION_GUARDIAN_SET_V1_PUBLIC_RECORD_AND_SOURCE_CHANGE_NO_ACTIVATION

## Public Guardian Set

| Field | Value |
| --- | --- |
| descriptor_type | production_guardian_set_v1 |
| project | xenchanted_x1_build_lab |
| network | x1_testnet |
| route | gateway_mint |
| guardian_set_version | 1 |
| guardian_count | 5 |
| threshold | 3 |
| quorum_model | 3-of-5 |
| key_type | Ed25519 / Solana public keys |
| ordering_rule | explicit_descriptor_order_guardian_01_to_guardian_05 |
| signature_domain | xxxl:x1-testnet:gateway-mint:v1:guardian-set-v1 |
| descriptor_hash_sha256 | 4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83 |
| effective_package | production-guardian-set-v1-public-record-and-source-change |

## Public Keys

| Guardian | Public Key |
| --- | --- |
| guardian_01 | `7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf` |
| guardian_02 | `GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp` |
| guardian_03 | `6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih` |
| guardian_04 | `9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY` |
| guardian_05 | `UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg` |

## Canonical Descriptor

Canonical descriptor:

    descriptor_type=production_guardian_set_v1
    project=xenchanted_x1_build_lab
    network=x1_testnet
    route=gateway_mint
    guardian_set_version=1
    guardian_count=5
    threshold=3
    quorum_model=3-of-5
    key_type=ed25519_solana_public_key
    ordering_rule=explicit_descriptor_order_guardian_01_to_guardian_05
    signature_domain=xxxl:x1-testnet:gateway-mint:v1:guardian-set-v1
    effective_package=production-guardian-set-v1-public-record-and-source-change
    guardian_01_public_key=7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf
    guardian_02_public_key=GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp
    guardian_03_public_key=6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih
    guardian_04_public_key=9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY
    guardian_05_public_key=UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg

## Descriptor Hash / Guardian Set ID

The descriptor hash is used as the source-bound guardian set id for V1.

    descriptor_hash_sha256=4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83

## Safety Boundary

Only public keys are recorded here.

Forbidden material:

- private keys;
- keypair JSON files;
- seed phrases;
- mnemonics;
- wallet exports;
- signing packages.

This package does not authorize activation, deploy, upgrade, RPC mutation, route enablement, SPL CPI enablement, proof log instantiation, or exact activation GO.

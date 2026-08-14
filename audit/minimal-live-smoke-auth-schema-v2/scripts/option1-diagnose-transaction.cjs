#!/usr/bin/env node
"use strict";

const fs = require("fs");
const crypto = require("crypto");
const {
  PublicKey,
  Transaction,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} = require("@solana/web3.js");

const TX_SOURCE_DIR = process.env.TX_SOURCE_DIR;
const EVIDENCE_DIR = process.env.EVIDENCE_DIR || process.cwd();

const PROGRAM_ID = new PublicKey("D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my");
const TARGET_SPL_MINT = new PublicKey("g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM");
const RECIPIENT_OWNER = new PublicKey("DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc");
const RECIPIENT_ATA = new PublicKey("9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k");
const MINT_AUTHORITY_PDA = new PublicKey("BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG");
const GATEWAY_CONFIG_PDA = new PublicKey("G79vy8cNu4eLoK42Aj15KJ7cW9n27DQxFkfYf8qcb67D");
const GUARDIAN_SET_PDA = new PublicKey("4yNzJ6cB6ecAovH2e12p2SC54WUio7HbThLqSRPMwuba");
const MINT_STATE_PDA = new PublicKey("57GckP3TXGQmyuFh6KcHqhL7NbsXuRdwW741FaJQtfQG");
const RECIPIENT_BALANCE_PDA = new PublicKey("5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB");
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

const EXPECTED_CONSUME_KEYS = [
  { name: "mint_state", pubkey: MINT_STATE_PDA, writable: false, signer: false },
  { name: "gateway_config", pubkey: GATEWAY_CONFIG_PDA, writable: false, signer: false },
  { name: "guardian_set", pubkey: GUARDIAN_SET_PDA, writable: false, signer: false },
  { name: "processed_event", pubkey: null, writable: true, signer: false },
  { name: "recipient_balance", pubkey: RECIPIENT_BALANCE_PDA, writable: true, signer: false },
  { name: "spl_token_mint", pubkey: TARGET_SPL_MINT, writable: true, signer: false },
  { name: "recipient_token_account", pubkey: RECIPIENT_ATA, writable: true, signer: false },
  { name: "mint_authority_pda", pubkey: MINT_AUTHORITY_PDA, writable: false, signer: false },
  { name: "token_program", pubkey: TOKEN_PROGRAM_ID, writable: false, signer: false },
  { name: "rent_payer", pubkey: RECIPIENT_OWNER, writable: true, signer: true },
  { name: "system_program", pubkey: SystemProgram.programId, writable: false, signer: false },
  { name: "instructions_sysvar", pubkey: SYSVAR_INSTRUCTIONS_PUBKEY, writable: false, signer: false },
];

const CONSUME_DISCRIMINATOR_HEX = "f2f4a868bb89fe52";
const ROUTE_ID_HEX = "d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c";
const GUARDIAN_SET_ID_HEX = "4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83";
const LEGACY_MINT_ID_HEX = "479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458";
const CANONICAL_EVENT_KEY_HEX = "e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102";

function sha256(buf) { return crypto.createHash("sha256").update(buf).digest("hex"); }
function b58From32(buf) { return new PublicKey(buf).toBase58(); }
function readU128LE(buf, offset) {
  let x = 0n;
  for (let i = 15; i >= 0; i--) x = (x << 8n) + BigInt(buf[offset + i]);
  return x.toString();
}
function printKV(k, v) {
  if (typeof v === "string" || typeof v === "number" || typeof v === "boolean") console.log(`${k}=${v}`);
  else console.log(`${k}=${JSON.stringify(v)}`);
}

function loadTx() {
  if (!TX_SOURCE_DIR) throw new Error("TX_SOURCE_DIR env missing");
  const path = `${TX_SOURCE_DIR}/results/option1_transaction_bytes.b64`;
  const b64 = fs.readFileSync(path, "utf8").trim();
  return { path, raw: Buffer.from(b64, "base64") };
}

function main() {
  const { path, raw } = loadTx();
  const tx = Transaction.from(raw);
  const ixCount = tx.instructions.length;
  const consumeIndex = 3;
  const consumeIx = tx.instructions[consumeIndex];
  if (!consumeIx) throw new Error(`consume instruction index ${consumeIndex} missing; ixCount=${ixCount}`);

  const data = Buffer.from(consumeIx.data);
  const keyDiagnostics = consumeIx.keys.map((k, index) => {
    const expected = EXPECTED_CONSUME_KEYS[index] || { name: `unexpected_${index}`, pubkey: null, writable: null, signer: null };
    return {
      index,
      expected_name: expected.name,
      pubkey: k.pubkey.toBase58(),
      expected_pubkey: expected.pubkey ? expected.pubkey.toBase58() : "dynamic_processed_event",
      pubkey_matches: expected.pubkey ? k.pubkey.equals(expected.pubkey) : true,
      isWritable: k.isWritable,
      expectedWritable: expected.writable,
      writable_matches: expected.writable === null ? true : k.isWritable === expected.writable,
      isSigner: k.isSigner,
      expectedSigner: expected.signer,
      signer_matches: expected.signer === null ? true : k.isSigner === expected.signer,
    };
  });

  const flagMismatches = keyDiagnostics.filter(k => !k.pubkey_matches || !k.writable_matches || !k.signer_matches);

  const decoded = {
    data_len: data.length,
    discriminator_hex: data.subarray(0, 8).toString("hex"),
    layout_version: data.readUInt16LE(8),
    account_meta_count: data[10],
    route_account_index: data[11],
    guardian_set_account_index: data[12],
    mint_state_account_index: data[13],
    processed_event_account_index: data[14],
    recipient_balance_account_index: data[15],
    route_id_hex: data.subarray(16, 48).toString("hex"),
    guardian_set_id_hex: data.subarray(48, 80).toString("hex"),
    mint_field_hex: data.subarray(80, 112).toString("hex"),
    mint_field_base58: b58From32(data.subarray(80, 112)),
    canonical_event_key_hex: data.subarray(112, 144).toString("hex"),
    recipient_field_hex: data.subarray(144, 176).toString("hex"),
    recipient_field_base58: b58From32(data.subarray(144, 176)),
    amount_u128_le: readU128LE(data, 176),
    source_chain_weight_bps: data.readUInt16LE(192),
    source_chain_id_u64: data.readBigUInt64LE(194).toString(),
    reserved_202_208_zero: data.subarray(202, 208).every(b => b === 0),
  };

  const targetSplMintHex = TARGET_SPL_MINT.toBuffer().toString("hex");
  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-diagnose-transaction-no-execution",
    evidence_dir: EVIDENCE_DIR,
    tx_source_dir: TX_SOURCE_DIR,
    tx_bytes_path: path,
    transaction_sha256: sha256(raw),
    transactions_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
    instruction_count: ixCount,
    ed25519_instruction_count_assuming_consume_index_3: consumeIndex,
    consume_instruction_index: consumeIndex,
    consume_program_id: consumeIx.programId.toBase58(),
    consume_program_matches: consumeIx.programId.equals(PROGRAM_ID),
    consume_key_count: consumeIx.keys.length,
    account_contract_flags_match: flagMismatches.length === 0,
    account_contract_mismatches: flagMismatches,
    decoded_consume_data: decoded,
    consume_data_layout_ok:
      decoded.data_len === 208 &&
      decoded.discriminator_hex === CONSUME_DISCRIMINATOR_HEX &&
      decoded.layout_version === 2 &&
      decoded.account_meta_count === 11 &&
      decoded.route_account_index === 1 &&
      decoded.guardian_set_account_index === 2 &&
      decoded.mint_state_account_index === 0 &&
      decoded.processed_event_account_index === 3 &&
      decoded.recipient_balance_account_index === 4 &&
      decoded.reserved_202_208_zero,
    route_id_matches: decoded.route_id_hex === ROUTE_ID_HEX,
    guardian_set_id_matches: decoded.guardian_set_id_hex === GUARDIAN_SET_ID_HEX,
    canonical_event_key_matches_dummy_burn_hash: decoded.canonical_event_key_hex === CANONICAL_EVENT_KEY_HEX,
    recipient_field_matches_recipient_owner: decoded.recipient_field_base58 === RECIPIENT_OWNER.toBase58(),
    mint_field_equals_legacy_mint_id: decoded.mint_field_hex === LEGACY_MINT_ID_HEX,
    mint_field_equals_target_spl_mint: decoded.mint_field_hex === targetSplMintHex,
    target_spl_mint_hex: targetSplMintHex,
    likely_current_failure: flagMismatches.length ? "account_contract_flag_or_key_mismatch" : "not_account_contract_flags",
    likely_next_failure_if_flags_fixed: decoded.mint_field_hex !== targetSplMintHex ? "mint_field_is_legacy_mint_id_not_target_spl_mint_possible_cpi_planning_mismatch" : "none_detected_from_mint_field",
  };

  fs.mkdirSync(`${EVIDENCE_DIR}/results`, { recursive: true });
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_diagnose_transaction_result.json`, JSON.stringify(result, null, 2));
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_consume_key_diagnostics.json`, JSON.stringify(keyDiagnostics, null, 2));

  console.log("=== option1 diagnose transaction compact summary for chat ===");
  const orderedKeys = [
    "package", "evidence_dir", "tx_source_dir", "tx_bytes_path", "transaction_sha256",
    "transactions_executed", "deploy_executed", "upgrade_executed", "push_executed",
    "instruction_count", "ed25519_instruction_count_assuming_consume_index_3", "consume_instruction_index",
    "consume_program_matches", "consume_key_count", "account_contract_flags_match", "account_contract_mismatches",
    "consume_data_layout_ok", "route_id_matches", "guardian_set_id_matches", "canonical_event_key_matches_dummy_burn_hash",
    "recipient_field_matches_recipient_owner", "mint_field_equals_legacy_mint_id", "mint_field_equals_target_spl_mint",
    "decoded_consume_data", "likely_current_failure", "likely_next_failure_if_flags_fixed",
  ];
  for (const key of orderedKeys) printKV(key, result[key]);
}

try { main(); } catch (err) {
  console.log("=== option1 diagnose transaction compact summary for chat ===");
  printKV("package", "runtime-state-provisioning-minimal-live-smoke-option1-diagnose-transaction-no-execution");
  printKV("diagnose_failed", true);
  printKV("error", err && err.message ? err.message : String(err));
  printKV("transactions_executed", false);
  printKV("deploy_executed", false);
  printKV("upgrade_executed", false);
  printKV("push_executed", false);
  process.exitCode = 1;
}

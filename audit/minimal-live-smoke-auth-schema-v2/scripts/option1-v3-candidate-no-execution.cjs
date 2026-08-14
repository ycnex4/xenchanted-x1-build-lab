#!/usr/bin/env node
"use strict";

const fs = require("fs");
const crypto = require("crypto");
const nacl = require("tweetnacl");
const {
  PublicKey,
  Keypair,
  Transaction,
  TransactionInstruction,
  Ed25519Program,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} = require("@solana/web3.js");

const EVIDENCE_DIR = process.env.EVIDENCE_DIR || process.cwd();
const PAYER_KEYPAIR = process.env.PAYER_KEYPAIR || process.env.XXXL_PAYER_KEYPAIR;

const PROGRAM_ID = new PublicKey("D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my");
const EXPECTED_PAYER = new PublicKey("DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc");
const TARGET_SPL_MINT = new PublicKey("g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM");
const RECIPIENT_OWNER = new PublicKey("DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc");
const RECIPIENT_ATA = new PublicKey("9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k");
const MINT_AUTHORITY_PDA = new PublicKey("BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG");

const GATEWAY_CONFIG_PDA = new PublicKey("G79vy8cNu4eLoK42Aj15KJ7cW9n27DQxFkfYf8qcb67D");
const GUARDIAN_SET_PDA = new PublicKey("4yNzJ6cB6ecAovH2e12p2SC54WUio7HbThLqSRPMwuba");
const MINT_STATE_PDA = new PublicKey("57GckP3TXGQmyuFh6KcHqhL7NbsXuRdwW741FaJQtfQG");
const RECIPIENT_BALANCE_PDA = new PublicKey("5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB");
const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

const ROUTE_ID = Buffer.from("d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c", "hex");
const GUARDIAN_SET_ID = Buffer.from("4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83", "hex");
const LEGACY_MINT_ID = Buffer.from("479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458", "hex");
const CANDIDATE_MINT_FIELD = TARGET_SPL_MINT.toBuffer();
const CANONICAL_EVENT_KEY = Buffer.from("e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102", "hex");

const CONSUME_GATEWAY_MINT_DISCRIMINATOR = Buffer.from([0xf2, 0xf4, 0xa8, 0x68, 0xbb, 0x89, 0xfe, 0x52]);
const INSTRUCTION_LAYOUT_VERSION = 2;
const CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT = 11;
const SOURCE_CHAIN_ID_U64 = 1n;
const SOURCE_CHAIN_WEIGHT_BPS = 10000;
const AMOUNT_ATOMIC = 1n;
const DUMMY_RECENT_BLOCKHASH = "11111111111111111111111111111111";

const EXPECTED_GUARDIANS = [
  "7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf",
  "GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp",
  "6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih",
];

const GUARDIAN_ENV_GROUPS = [
  ["XXXL_GUARDIAN_0_KEYPAIR", "XXXL_GUARDIAN_KEYPAIR_0", "GUARDIAN_0_KEYPAIR", "GUARDIAN_KEYPAIR_0"],
  ["XXXL_GUARDIAN_1_KEYPAIR", "XXXL_GUARDIAN_KEYPAIR_1", "GUARDIAN_1_KEYPAIR", "GUARDIAN_KEYPAIR_1"],
  ["XXXL_GUARDIAN_2_KEYPAIR", "XXXL_GUARDIAN_KEYPAIR_2", "GUARDIAN_2_KEYPAIR", "GUARDIAN_KEYPAIR_2"],
];

function sha256(buf) {
  return crypto.createHash("sha256").update(buf).digest();
}

function u64le(value) {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(BigInt(value), 0);
  return b;
}

function writeU128LE(buf, offset, value) {
  let x = BigInt(value);
  for (let i = 0; i < 16; i++) {
    buf[offset + i] = Number((x >> (8n * BigInt(i))) & 0xffn);
  }
}

function readU128LE(buf, offset) {
  let x = 0n;
  for (let i = 15; i >= 0; i--) x = (x << 8n) + BigInt(buf[offset + i]);
  return x.toString();
}

function readKeypairNoSecretEcho(path) {
  const arr = JSON.parse(fs.readFileSync(path, "utf8"));
  const secret = Uint8Array.from(arr);
  if (secret.length === 64) return Keypair.fromSecretKey(secret);
  if (secret.length === 32) return Keypair.fromSeed(secret);
  throw new Error(`unsupported keypair length ${secret.length}`);
}

function findEnv(names) {
  for (const name of names) {
    if (process.env[name]) return { name, path: process.env[name] };
  }
  return null;
}

function readGuardianKeypair(index) {
  const found = findEnv(GUARDIAN_ENV_GROUPS[index]);
  if (!found) throw new Error(`guardian ${index} keypair env missing`);
  const kp = readKeypairNoSecretEcho(found.path);
  const pubkey = kp.publicKey.toBase58();
  if (pubkey !== EXPECTED_GUARDIANS[index]) {
    throw new Error(`guardian ${index} pubkey mismatch: got ${pubkey}, expected ${EXPECTED_GUARDIANS[index]}`);
  }
  return { kp, envName: found.name };
}

function buildPayloadHash(processedEventPda) {
  return sha256(Buffer.concat([
    Buffer.from("consume_gateway_mint_authorization_v2"),
    processedEventPda.toBuffer(),
    ROUTE_ID,
    TARGET_SPL_MINT.toBuffer(),
    RECIPIENT_ATA.toBuffer(),
    u64le(AMOUNT_ATOMIC),
    GUARDIAN_SET_ID,
  ]));
}

function buildConsumeDataV3Candidate() {
  const data = Buffer.alloc(208);
  CONSUME_GATEWAY_MINT_DISCRIMINATOR.copy(data, 0);
  data.writeUInt16LE(INSTRUCTION_LAYOUT_VERSION, 8);
  data[10] = CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT;
  data[11] = 1;
  data[12] = 2;
  data[13] = 0;
  data[14] = 3;
  data[15] = 4;
  ROUTE_ID.copy(data, 16);
  GUARDIAN_SET_ID.copy(data, 48);
  CANDIDATE_MINT_FIELD.copy(data, 80);
  CANONICAL_EVENT_KEY.copy(data, 112);
  RECIPIENT_OWNER.toBuffer().copy(data, 144);
  writeU128LE(data, 176, AMOUNT_ATOMIC);
  data.writeUInt16LE(SOURCE_CHAIN_WEIGHT_BPS, 192);
  data.writeBigUInt64LE(SOURCE_CHAIN_ID_U64, 194);
  return data;
}

function decodeConsumeData(data) {
  return {
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
    mint_field_base58: new PublicKey(data.subarray(80, 112)).toBase58(),
    canonical_event_key_hex: data.subarray(112, 144).toString("hex"),
    recipient_field_hex: data.subarray(144, 176).toString("hex"),
    recipient_field_base58: new PublicKey(data.subarray(144, 176)).toBase58(),
    amount_u128_le: readU128LE(data, 176),
    source_chain_weight_bps: data.readUInt16LE(192),
    source_chain_id_u64: data.readBigUInt64LE(194).toString(),
    reserved_202_208_zero: data.subarray(202, 208).every(x => x === 0),
  };
}

function accountFlags(keys) {
  return keys.map((k, index) => ({
    index,
    pubkey: k.pubkey.toBase58(),
    isSigner: k.isSigner,
    isWritable: k.isWritable,
  }));
}

function accountContractCheck(keys) {
  const expected = [
    [false, false],
    [false, false],
    [false, false],
    [false, true],
    [false, true],
    [false, true],
    [false, true],
    [false, false],
    [false, false],
    [true, true],
    [false, false],
    [false, false],
  ];
  const mismatches = [];
  keys.forEach((k, i) => {
    const e = expected[i];
    if (!e || k.isSigner !== e[0] || k.isWritable !== e[1]) {
      mismatches.push({ index: i, pubkey: k.pubkey.toBase58(), got: { signer: k.isSigner, writable: k.isWritable }, expected: e ? { signer: e[0], writable: e[1] } : "none" });
    }
  });
  return { ok: keys.length === 12 && mismatches.length === 0, mismatches };
}

function printKV(k, v) {
  if (typeof v === "string") console.log(`${k}=${v}`);
  else console.log(`${k}=${JSON.stringify(v)}`);
}

function buildCandidateTransaction(payer, guardians, processedEventPda, payloadHash, consumeData) {
  const ed25519Instructions = guardians.map(({ kp }) => {
    const signature = nacl.sign.detached(payloadHash, kp.secretKey);
    return Ed25519Program.createInstructionWithPublicKey({
      publicKey: kp.publicKey.toBytes(),
      message: payloadHash,
      signature,
    });
  });

  const consumeKeys = [
    { pubkey: MINT_STATE_PDA, isSigner: false, isWritable: false },
    { pubkey: GATEWAY_CONFIG_PDA, isSigner: false, isWritable: false },
    { pubkey: GUARDIAN_SET_PDA, isSigner: false, isWritable: false },
    { pubkey: processedEventPda, isSigner: false, isWritable: true },
    { pubkey: RECIPIENT_BALANCE_PDA, isSigner: false, isWritable: true },
    { pubkey: TARGET_SPL_MINT, isSigner: false, isWritable: true },
    { pubkey: RECIPIENT_ATA, isSigner: false, isWritable: true },
    { pubkey: MINT_AUTHORITY_PDA, isSigner: false, isWritable: false },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: payer.publicKey, isSigner: true, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_INSTRUCTIONS_PUBKEY, isSigner: false, isWritable: false },
  ];

  const consumeIx = new TransactionInstruction({ programId: PROGRAM_ID, keys: consumeKeys, data: consumeData });
  const tx = new Transaction();
  for (const ix of ed25519Instructions) tx.add(ix);
  tx.add(consumeIx);
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = DUMMY_RECENT_BLOCKHASH;
  tx.sign(payer);
  return { tx, ed25519Instructions, consumeIx, consumeKeys };
}

function main() {
  if (!PAYER_KEYPAIR) throw new Error("PAYER_KEYPAIR/XXXL_PAYER_KEYPAIR missing");
  const payer = readKeypairNoSecretEcho(PAYER_KEYPAIR);
  if (payer.publicKey.toBase58() !== EXPECTED_PAYER.toBase58()) {
    throw new Error(`payer pubkey mismatch: ${payer.publicKey.toBase58()}`);
  }

  const guardians = [0, 1, 2].map(readGuardianKeypair);
  const [processedEventPda, processedEventBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("processed-event"), CANONICAL_EVENT_KEY],
    PROGRAM_ID
  );

  const [mintStatePdaForLegacyMintId, legacyMintStateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("mint-state"), LEGACY_MINT_ID],
    PROGRAM_ID
  );
  const [mintStatePdaForTargetSplMint, targetSplMintStateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("mint-state"), TARGET_SPL_MINT.toBuffer()],
    PROGRAM_ID
  );

  const payloadHash = buildPayloadHash(processedEventPda);
  const consumeData = buildConsumeDataV3Candidate();
  const decoded = decodeConsumeData(consumeData);
  const { tx, consumeKeys } = buildCandidateTransaction(payer, guardians, processedEventPda, payloadHash, consumeData);
  const rawTx = tx.serialize();
  const acctCheck = accountContractCheck(consumeKeys);

  const candidateMintStatePdaMatchesAccount0 = mintStatePdaForTargetSplMint.equals(MINT_STATE_PDA);
  const legacyMintStatePdaMatchesAccount0 = mintStatePdaForLegacyMintId.equals(MINT_STATE_PDA);

  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-v3-candidate-no-execution",
    evidence_dir: EVIDENCE_DIR,
    program_id: PROGRAM_ID.toBase58(),
    transactions_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
    candidate_transaction_built: true,
    candidate_transaction_sha256: sha256(rawTx).toString("hex"),
    candidate_transaction_bytes_saved: `${EVIDENCE_DIR}/results/option1_v3_candidate_transaction_bytes.b64`,
    ed25519_instruction_count: 3,
    consume_instruction_index: 3,
    consume_key_count: consumeKeys.length,
    account_contract_flags_match: acctCheck.ok,
    account_contract_mismatches: acctCheck.mismatches,
    payload_account_meta_count: 11,
    real_account_count: 12,
    instructions_sysvar_account_index: 11,
    canonical_event_key_hex: CANONICAL_EVENT_KEY.toString("hex"),
    processed_event_pda: processedEventPda.toBase58(),
    processed_event_bump: processedEventBump,
    signed_message_source: "expected_payload_hash",
    signed_message_hex: payloadHash.toString("hex"),
    signed_message_expected_from_preflight: "02959e687c74756661cac356cc65bdff184df5810437a01558144c45384e2823",
    signed_message_matches_preflight: payloadHash.toString("hex") === "02959e687c74756661cac356cc65bdff184df5810437a01558144c45384e2823",
    candidate_mint_field_base58: decoded.mint_field_base58,
    mint_field_equals_target_spl_mint: decoded.mint_field_base58 === TARGET_SPL_MINT.toBase58(),
    mint_field_equals_legacy_mint_id: Buffer.from(decoded.mint_field_hex, "hex").equals(LEGACY_MINT_ID),
    legacy_mint_id_as_base58: new PublicKey(LEGACY_MINT_ID).toBase58(),
    target_spl_mint: TARGET_SPL_MINT.toBase58(),
    current_mint_state_pda_account0: MINT_STATE_PDA.toBase58(),
    mint_state_pda_for_legacy_mint_id: mintStatePdaForLegacyMintId.toBase58(),
    mint_state_pda_for_legacy_mint_id_bump: legacyMintStateBump,
    legacy_mint_state_pda_matches_account0: legacyMintStatePdaMatchesAccount0,
    mint_state_pda_for_target_spl_mint: mintStatePdaForTargetSplMint.toBase58(),
    mint_state_pda_for_target_spl_mint_bump: targetSplMintStateBump,
    candidate_mint_state_pda_matches_account0: candidateMintStatePdaMatchesAccount0,
    decoded_consume_data: decoded,
    candidate_account_flags: accountFlags(consumeKeys),
    no_execution_verdict: candidateMintStatePdaMatchesAccount0
      ? "candidate_structurally_ready_for_review_before_execution"
      : "candidate_not_ready_mint_state_pda_mismatch_if_runtime_derives_mint_state_from_args_mint_id",
    recommendation: candidateMintStatePdaMatchesAccount0
      ? "review_candidate_then_request_explicit_go_before_any_execution"
      : "do_not_execute_v3_candidate_without_source_or_state_strategy_review",
  };

  fs.mkdirSync(`${EVIDENCE_DIR}/results`, { recursive: true });
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_v3_candidate_result.json`, JSON.stringify(result, null, 2));
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_v3_candidate_transaction_bytes.b64`, rawTx.toString("base64"));
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_v3_candidate_consume_keys.json`, JSON.stringify(accountFlags(consumeKeys), null, 2));

  console.log("=== option1 v3 candidate no-execution compact summary for chat ===");
  const ordered = [
    "package", "evidence_dir", "program_id", "transactions_executed", "deploy_executed", "upgrade_executed", "push_executed",
    "candidate_transaction_built", "candidate_transaction_sha256", "ed25519_instruction_count", "consume_instruction_index",
    "consume_key_count", "account_contract_flags_match", "account_contract_mismatches",
    "canonical_event_key_hex", "processed_event_pda", "signed_message_hex", "signed_message_matches_preflight",
    "candidate_mint_field_base58", "mint_field_equals_target_spl_mint", "mint_field_equals_legacy_mint_id",
    "current_mint_state_pda_account0", "mint_state_pda_for_legacy_mint_id", "legacy_mint_state_pda_matches_account0",
    "mint_state_pda_for_target_spl_mint", "candidate_mint_state_pda_matches_account0",
    "no_execution_verdict", "recommendation", "candidate_transaction_bytes_saved",
  ];
  for (const k of ordered) printKV(k, result[k]);
}

try {
  main();
} catch (err) {
  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-v3-candidate-no-execution",
    candidate_failed: true,
    error: err && err.message ? err.message : String(err),
    transactions_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
  };
  console.log("=== option1 v3 candidate no-execution compact summary for chat ===");
  for (const [k, v] of Object.entries(result)) printKV(k, v);
  process.exitCode = 1;
}

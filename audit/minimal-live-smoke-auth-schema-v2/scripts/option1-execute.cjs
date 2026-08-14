#!/usr/bin/env node
"use strict";

const fs = require("fs");
const crypto = require("crypto");
const nacl = require("tweetnacl");
const {
  PublicKey,
  Connection,
  Keypair,
  Transaction,
  TransactionInstruction,
  Ed25519Program,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} = require("@solana/web3.js");

const RPC_URL = process.env.RPC_URL || process.env.SOLANA_RPC_URL;
const PAYER_KEYPAIR = process.env.PAYER_KEYPAIR || process.env.XXXL_PAYER_KEYPAIR;
const EVIDENCE_DIR = process.env.EVIDENCE_DIR || process.cwd();

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
const MINT_ID = Buffer.from("479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458", "hex");
const CANONICAL_EVENT_KEY = Buffer.from("e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102", "hex");

const CONSUME_GATEWAY_MINT_DISCRIMINATOR = Buffer.from([0xf2, 0xf4, 0xa8, 0x68, 0xbb, 0x89, 0xfe, 0x52]);
const INSTRUCTION_LAYOUT_VERSION = 2;
const CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT = 11;
const SOURCE_CHAIN_ID_U64 = 1n;
const SOURCE_CHAIN_WEIGHT_BPS = 10000;
const AMOUNT_ATOMIC = 1n;
const EXPECTED_CUSTOM_ERROR = 8; // CpiBoundaryNotReady
const EXPECTED_CONSUME_INSTRUCTION_INDEX = 3;

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

function readU64LE(data, offset) {
  if (!data || data.length < offset + 8) return "unreadable";
  return data.readBigUInt64LE(offset).toString();
}

function readU128LE(data, offset) {
  if (!data || data.length < offset + 16) return "unreadable";
  let x = 0n;
  for (let i = 15; i >= 0; i--) x = (x << 8n) + BigInt(data[offset + i]);
  return x.toString();
}

function writeU128LE(buf, offset, value) {
  let x = BigInt(value);
  for (let i = 0; i < 16; i++) {
    buf[offset + i] = Number((x >> (8n * BigInt(i))) & 0xffn);
  }
}

function u64le(value) {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(BigInt(value), 0);
  return b;
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

async function getAccount(connection, pubkey) {
  const info = await connection.getAccountInfo(pubkey, "confirmed");
  if (!info) return { exists: false, owner: "missing", lamports: 0, data_len: 0, data_sha256: "missing", data: null };
  return {
    exists: true,
    owner: info.owner.toBase58(),
    lamports: info.lamports,
    data_len: info.data.length,
    data_sha256: sha256(info.data).toString("hex"),
    data: info.data,
  };
}

async function snapshot(connection, processedEventPda) {
  const gatewayConfig = await getAccount(connection, GATEWAY_CONFIG_PDA);
  const guardianSet = await getAccount(connection, GUARDIAN_SET_PDA);
  const mintState = await getAccount(connection, MINT_STATE_PDA);
  const recipientBalance = await getAccount(connection, RECIPIENT_BALANCE_PDA);
  const processedEvent = await getAccount(connection, processedEventPda);
  const splMint = await getAccount(connection, TARGET_SPL_MINT);
  const recipientAta = await getAccount(connection, RECIPIENT_ATA);

  return {
    accounts: {
      gateway_config: compactAccount(gatewayConfig),
      guardian_set: compactAccount(guardianSet),
      mint_state: compactAccount(mintState),
      recipient_balance: compactAccount(recipientBalance),
      processed_event: compactAccount(processedEvent),
      spl_mint: compactAccount(splMint),
      recipient_ata: compactAccount(recipientAta),
    },
    values: {
      spl_mint_supply: readU64LE(splMint.data, 36),
      recipient_ata_amount: readU64LE(recipientAta.data, 64),
      mint_state_total_supply: readU128LE(mintState.data, 48),
      recipient_balance_amount: readU128LE(recipientBalance.data, 80),
      processed_event_exists: processedEvent.exists,
    },
    fullAccountDataForChecks: {
      guardianSetData: guardianSet.data,
      splMintData: splMint.data,
      recipientAtaData: recipientAta.data,
      mintStateData: mintState.data,
      recipientBalanceData: recipientBalance.data,
    },
  };
}

function compactAccount(a) {
  return {
    exists: a.exists,
    owner: a.owner,
    lamports: a.lamports,
    data_len: a.data_len,
    data_sha256: a.data_sha256,
  };
}

function compareAccounts(before, after) {
  const changed = [];
  for (const label of Object.keys(before.accounts)) {
    const b = before.accounts[label];
    const a = after.accounts[label];
    if (
      b.exists !== a.exists ||
      b.owner !== a.owner ||
      b.lamports !== a.lamports ||
      b.data_len !== a.data_len ||
      b.data_sha256 !== a.data_sha256
    ) {
      changed.push(label);
    }
  }
  return changed;
}

function extractInstructionCustomErr(err) {
  if (!err || !err.InstructionError || !Array.isArray(err.InstructionError)) return null;
  const [index, inner] = err.InstructionError;
  if (inner && typeof inner === "object" && Object.prototype.hasOwnProperty.call(inner, "Custom")) {
    return { index, custom: inner.Custom };
  }
  return { index, custom: null, raw: inner };
}

function buildConsumeData() {
  const data = Buffer.alloc(208);
  CONSUME_GATEWAY_MINT_DISCRIMINATOR.copy(data, 0);
  data.writeUInt16LE(INSTRUCTION_LAYOUT_VERSION, 8);
  data[10] = CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT;
  data[11] = 1; // route/gateway_config account index
  data[12] = 2; // guardian_set account index
  data[13] = 0; // mint_state account index
  data[14] = 3; // processed_event account index
  data[15] = 4; // recipient_balance account index
  ROUTE_ID.copy(data, 16);
  GUARDIAN_SET_ID.copy(data, 48);
  MINT_ID.copy(data, 80);
  CANONICAL_EVENT_KEY.copy(data, 112);
  RECIPIENT_OWNER.toBuffer().copy(data, 144);
  writeU128LE(data, 176, AMOUNT_ATOMIC);
  data.writeUInt16LE(SOURCE_CHAIN_WEIGHT_BPS, 192);
  data.writeBigUInt64LE(SOURCE_CHAIN_ID_U64, 194);
  // bytes 202..208 intentionally remain zero-reserved.
  return data;
}

function buildPayloadHash(processedEventPda) {
  return sha256(Buffer.concat([
    Buffer.from("consume_gateway_mint_authorization_v2", "utf8"),
    processedEventPda.toBuffer(),
    ROUTE_ID,
    TARGET_SPL_MINT.toBuffer(),
    RECIPIENT_ATA.toBuffer(),
    u64le(AMOUNT_ATOMIC),
    GUARDIAN_SET_ID,
  ]));
}

async function waitForTransaction(connection, signature) {
  let status = null;
  let tx = null;
  for (let i = 0; i < 75; i++) {
    const statuses = await connection.getSignatureStatuses([signature], { searchTransactionHistory: true });
    status = statuses && statuses.value ? statuses.value[0] : null;
    try {
      tx = await connection.getTransaction(signature, { commitment: "confirmed", maxSupportedTransactionVersion: 0 });
    } catch (_) {
      tx = null;
    }
    if (status && tx) break;
    await new Promise(resolve => setTimeout(resolve, 2000));
  }
  return { status, tx };
}

function printKV(k, v) {
  if (typeof v === "string" || typeof v === "number" || typeof v === "boolean") console.log(`${k}=${v}`);
  else console.log(`${k}=${JSON.stringify(v)}`);
}

async function main() {
  if (!RPC_URL) throw new Error("RPC_URL/SOLANA_RPC_URL missing");
  if (/mainnet/i.test(RPC_URL)) throw new Error(`RPC_URL looks like mainnet: ${RPC_URL}`);
  if (!PAYER_KEYPAIR) throw new Error("PAYER_KEYPAIR/XXXL_PAYER_KEYPAIR missing");

  const connection = new Connection(RPC_URL, "confirmed");
  const payer = readKeypairNoSecretEcho(PAYER_KEYPAIR);
  if (payer.publicKey.toBase58() !== EXPECTED_PAYER.toBase58()) {
    throw new Error(`payer pubkey mismatch: ${payer.publicKey.toBase58()}`);
  }

  const guardians = [0, 1, 2].map(readGuardianKeypair);

  const [processedEventPda, processedEventBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("processed-event"), CANONICAL_EVENT_KEY],
    PROGRAM_ID
  );

  const before = await snapshot(connection, processedEventPda);
  fs.mkdirSync(`${EVIDENCE_DIR}/results`, { recursive: true });
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_execute_pre_snapshot.json`, JSON.stringify(before, null, 2));

  const prestateZero =
    before.values.spl_mint_supply === "0" &&
    before.values.recipient_ata_amount === "0" &&
    before.values.mint_state_total_supply === "0" &&
    before.values.recipient_balance_amount === "0";
  if (!prestateZero) throw new Error(`prestate not zero: ${JSON.stringify(before.values)}`);

  const payloadHash = buildPayloadHash(processedEventPda);
  const consumeData = buildConsumeData();

  const ed25519Instructions = guardians.map(({ kp }) => {
    const signature = nacl.sign.detached(payloadHash, kp.secretKey);
    return Ed25519Program.createInstructionWithPublicKey({
      publicKey: kp.publicKey.toBytes(),
      message: payloadHash,
      signature,
    });
  });

  const consumeIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: MINT_STATE_PDA, isSigner: false, isWritable: true },
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
    ],
    data: consumeData,
  });

  const tx = new Transaction();
  for (const ix of ed25519Instructions) tx.add(ix);
  tx.add(consumeIx);
  tx.feePayer = payer.publicKey;
  const latest = await connection.getLatestBlockhash("confirmed");
  tx.recentBlockhash = latest.blockhash;
  tx.sign(payer);

  const rawTx = tx.serialize();
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_transaction_bytes.b64`, rawTx.toString("base64"));

  let signature;
  try {
    signature = await connection.sendRawTransaction(rawTx, { skipPreflight: true, maxRetries: 3 });
  } catch (err) {
    const result = {
      package: "runtime-state-provisioning-minimal-live-smoke-option1-structure-only-execution",
      evidence_dir: EVIDENCE_DIR,
      rpc_url: RPC_URL,
      program_id: PROGRAM_ID.toBase58(),
      send_failed: true,
      send_error: err && err.message ? err.message : String(err),
      transactions_executed: false,
      deploy_executed: false,
      upgrade_executed: false,
      push_executed: false,
    };
    fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_execute_result.json`, JSON.stringify(result, null, 2));
    console.log("=== option1 execution compact summary for chat ===");
    for (const [k, v] of Object.entries(result)) printKV(k, v);
    process.exitCode = 1;
    return;
  }

  const waited = await waitForTransaction(connection, signature);
  const err = waited.tx && waited.tx.meta ? waited.tx.meta.err : waited.status ? waited.status.err : null;
  const customErr = extractInstructionCustomErr(err);
  const logs = waited.tx && waited.tx.meta && Array.isArray(waited.tx.meta.logMessages) ? waited.tx.meta.logMessages : [];
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_tx_logs.txt`, logs.join("\n") + (logs.length ? "\n" : ""));

  const after = await snapshot(connection, processedEventPda);
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_execute_post_snapshot.json`, JSON.stringify(after, null, 2));

  const changedAccounts = compareAccounts(before, after);
  const balancesUnchanged =
    before.values.spl_mint_supply === after.values.spl_mint_supply &&
    before.values.recipient_ata_amount === after.values.recipient_ata_amount &&
    before.values.mint_state_total_supply === after.values.mint_state_total_supply &&
    before.values.recipient_balance_amount === after.values.recipient_balance_amount;

  const failureMatches = Boolean(
    customErr &&
    customErr.index === EXPECTED_CONSUME_INSTRUCTION_INDEX &&
    customErr.custom === EXPECTED_CUSTOM_ERROR
  );

  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-structure-only-execution",
    evidence_dir: EVIDENCE_DIR,
    rpc_url: RPC_URL,
    program_id: PROGRAM_ID.toBase58(),
    tx_signature: signature,
    tx_landed: Boolean(waited.status),
    tx_confirmation_status: waited.status ? waited.status.confirmationStatus : "null",
    tx_slot: waited.status ? waited.status.slot : "null",
    tx_meta_available: Boolean(waited.tx && waited.tx.meta),
    tx_err: err,
    custom_error_instruction_index: customErr ? customErr.index : "null",
    custom_error_code: customErr ? customErr.custom : "null",
    expected_failure: "CpiBoundaryNotReady",
    expected_custom_error: EXPECTED_CUSTOM_ERROR,
    expected_consume_instruction_index: EXPECTED_CONSUME_INSTRUCTION_INDEX,
    failure_code_matches_cpi_boundary: failureMatches,
    canonical_event_key_hex: CANONICAL_EVENT_KEY.toString("hex"),
    processed_event_pda: processedEventPda.toBase58(),
    processed_event_bump: processedEventBump,
    signed_message_source: "expected_payload_hash",
    signed_message_hex: payloadHash.toString("hex"),
    guardian_pubkeys_used: guardians.map(g => g.kp.publicKey.toBase58()),
    ed25519_instruction_count: 3,
    consume_instruction_index: 3,
    payload_account_meta_count: 11,
    real_account_count: 12,
    instructions_sysvar_account_index: 11,
    before_values: before.values,
    after_values: after.values,
    changed_accounts: changedAccounts,
    state_and_token_accounts_unchanged: changedAccounts.length === 0,
    supply_and_balance_values_unchanged: balancesUnchanged,
    option1_success: Boolean(waited.status) && failureMatches && changedAccounts.length === 0 && balancesUnchanged,
    logs_saved: `${EVIDENCE_DIR}/results/option1_tx_logs.txt`,
    transaction_bytes_saved: `${EVIDENCE_DIR}/results/option1_transaction_bytes.b64`,
    transactions_executed: true,
    consume_gateway_mint_transaction_executed: true,
    processed_event_mutation_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
  };

  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_execute_result.json`, JSON.stringify(result, null, 2));

  console.log("=== option1 execution compact summary for chat ===");
  const orderedKeys = [
    "package", "evidence_dir", "rpc_url", "program_id", "tx_signature", "tx_landed",
    "tx_confirmation_status", "tx_slot", "tx_meta_available", "tx_err",
    "custom_error_instruction_index", "custom_error_code", "expected_failure",
    "failure_code_matches_cpi_boundary", "canonical_event_key_hex", "processed_event_pda",
    "signed_message_source", "signed_message_hex", "ed25519_instruction_count",
    "consume_instruction_index", "payload_account_meta_count", "real_account_count",
    "instructions_sysvar_account_index", "before_values", "after_values", "changed_accounts",
    "state_and_token_accounts_unchanged", "supply_and_balance_values_unchanged", "option1_success",
    "logs_saved", "transaction_bytes_saved", "transactions_executed",
    "consume_gateway_mint_transaction_executed", "processed_event_mutation_executed",
    "deploy_executed", "upgrade_executed", "push_executed",
  ];
  for (const k of orderedKeys) printKV(k, result[k]);
}

main().catch(err => {
  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-structure-only-execution",
    evidence_dir: EVIDENCE_DIR,
    fatal_error: err && err.message ? err.message : String(err),
    transactions_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
  };
  try {
    fs.mkdirSync(`${EVIDENCE_DIR}/results`, { recursive: true });
    fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_execute_result.json`, JSON.stringify(result, null, 2));
  } catch (_) {}
  console.log("=== option1 execution compact summary for chat ===");
  for (const [k, v] of Object.entries(result)) printKV(k, v);
  process.exitCode = 1;
});

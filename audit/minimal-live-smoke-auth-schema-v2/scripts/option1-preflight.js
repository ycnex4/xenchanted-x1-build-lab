#!/usr/bin/env node
"use strict";

const fs = require("fs");
const crypto = require("crypto");
const { PublicKey, Connection, Keypair } = require("@solana/web3.js");

const RPC_URL = process.env.RPC_URL || process.env.SOLANA_RPC_URL;
const PAYER_KEYPAIR = process.env.PAYER_KEYPAIR || process.env.XXXL_PAYER_KEYPAIR;
const EVIDENCE_DIR = process.env.EVIDENCE_DIR || process.cwd();

const PROGRAM_ID = new PublicKey("D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my");
const EXPECTED_PAYER = new PublicKey("DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc");
const TARGET_SPL_MINT = new PublicKey("g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM");
const RECIPIENT_OWNER = new PublicKey("DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc");
const RECIPIENT_ATA = new PublicKey("9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k");
const GATEWAY_CONFIG_PDA = new PublicKey("G79vy8cNu4eLoK42Aj15KJ7cW9n27DQxFkfYf8qcb67D");
const GUARDIAN_SET_PDA = new PublicKey("4yNzJ6cB6ecAovH2e12p2SC54WUio7HbThLqSRPMwuba");
const MINT_STATE_PDA = new PublicKey("57GckP3TXGQmyuFh6KcHqhL7NbsXuRdwW741FaJQtfQG");
const RECIPIENT_BALANCE_PDA = new PublicKey("5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB");

const ROUTE_ID = Buffer.from("d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c", "hex");
const GUARDIAN_SET_ID = Buffer.from("4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83", "hex");
const MINT_ID = Buffer.from("479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458", "hex");
const CANONICAL_EVENT_KEY = Buffer.from("e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102", "hex");

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

function u64le(n) {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(BigInt(n), 0);
  return b;
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

function readKeypairNoSecretEcho(path) {
  const raw = fs.readFileSync(path, "utf8");
  const arr = JSON.parse(raw);
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

function checkKeypairEnv(index) {
  const found = findEnv(GUARDIAN_ENV_GROUPS[index]);
  if (!found) {
    return { index, env_present: false, env_name: "none", readable: false, pubkey: "none", pubkey_matches: false };
  }
  try {
    const kp = readKeypairNoSecretEcho(found.path);
    const pubkey = kp.publicKey.toBase58();
    return {
      index,
      env_present: true,
      env_name: found.name,
      readable: true,
      pubkey,
      pubkey_matches: pubkey === EXPECTED_GUARDIANS[index],
    };
  } catch (err) {
    return {
      index,
      env_present: true,
      env_name: found.name,
      readable: false,
      pubkey: "unreadable",
      pubkey_matches: false,
      error: err.message,
    };
  }
}

function checkPayer() {
  if (!PAYER_KEYPAIR) return { readable: false, pubkey: "none", pubkey_matches_expected: false };
  try {
    const kp = readKeypairNoSecretEcho(PAYER_KEYPAIR);
    const pubkey = kp.publicKey.toBase58();
    return { readable: true, pubkey, pubkey_matches_expected: pubkey === EXPECTED_PAYER.toBase58() };
  } catch (err) {
    return { readable: false, pubkey: "unreadable", pubkey_matches_expected: false, error: err.message };
  }
}

async function getAccount(connection, pubkey) {
  const info = await connection.getAccountInfo(pubkey, "confirmed");
  if (!info) return { exists: false, owner: "missing", data_len: 0, data_sha256: "missing" };
  return {
    exists: true,
    owner: info.owner.toBase58(),
    data_len: info.data.length,
    data_sha256: sha256(info.data).toString("hex"),
    data: info.data,
  };
}

function printKV(k, v) {
  if (typeof v === "string" || typeof v === "number" || typeof v === "boolean") {
    console.log(`${k}=${v}`);
  } else {
    console.log(`${k}=${JSON.stringify(v)}`);
  }
}

async function main() {
  const result = {
    package: "runtime-state-provisioning-minimal-live-smoke-option1-builder-preflight-no-execution",
    evidence_dir: EVIDENCE_DIR,
    rpc_url: RPC_URL || "missing",
    program_id: PROGRAM_ID.toBase58(),
    transactions_executed: false,
    deploy_executed: false,
    upgrade_executed: false,
    push_executed: false,
  };

  if (!RPC_URL) throw new Error("RPC_URL/SOLANA_RPC_URL missing");
  if (/mainnet/i.test(RPC_URL)) throw new Error(`RPC_URL looks like mainnet: ${RPC_URL}`);

  const connection = new Connection(RPC_URL, "confirmed");

  const payerCheck = checkPayer();
  const guardianChecks = [0, 1, 2].map(checkKeypairEnv);

  const [processedEventPda, processedEventBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("processed-event"), CANONICAL_EVENT_KEY],
    PROGRAM_ID
  );

  const [gatewayConfigDerived] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("gateway-config"), ROUTE_ID],
    PROGRAM_ID
  );
  const [guardianSetDerived] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("guardian-set"), GUARDIAN_SET_ID],
    PROGRAM_ID
  );
  const [mintStateDerived] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("mint-state"), MINT_ID],
    PROGRAM_ID
  );
  const [recipientBalanceDerived] = PublicKey.findProgramAddressSync(
    [Buffer.from("xxxl"), Buffer.from("recipient-balance"), RECIPIENT_OWNER.toBuffer(), TARGET_SPL_MINT.toBuffer()],
    PROGRAM_ID
  );

  const signedMessage = sha256(Buffer.concat([
    Buffer.from("consume_gateway_mint_authorization_v2", "utf8"),
    processedEventPda.toBuffer(),
    ROUTE_ID,
    TARGET_SPL_MINT.toBuffer(),
    RECIPIENT_ATA.toBuffer(),
    u64le(1),
    GUARDIAN_SET_ID,
  ]));

  const splMint = await getAccount(connection, TARGET_SPL_MINT);
  const recipientAta = await getAccount(connection, RECIPIENT_ATA);
  const mintState = await getAccount(connection, MINT_STATE_PDA);
  const recipientBalance = await getAccount(connection, RECIPIENT_BALANCE_PDA);
  const gatewayConfig = await getAccount(connection, GATEWAY_CONFIG_PDA);
  const guardianSet = await getAccount(connection, GUARDIAN_SET_PDA);
  const processedEvent = await getAccount(connection, processedEventPda);

  const onchainGuardianPubkeys = [];
  if (guardianSet.exists && guardianSet.data_len >= 176) {
    const count = guardianSet.data[14];
    for (let i = 0; i < count; i++) {
      onchainGuardianPubkeys.push(new PublicKey(guardianSet.data.subarray(16 + i * 32, 16 + (i + 1) * 32)).toBase58());
    }
  }

  const expectedGuardiansOnchain = EXPECTED_GUARDIANS.every(g => onchainGuardianPubkeys.includes(g));
  const prestateZero =
    readU64LE(splMint.data, 36) === "0" &&
    readU64LE(recipientAta.data, 64) === "0" &&
    readU128LE(mintState.data, 48) === "0" &&
    readU128LE(recipientBalance.data, 80) === "0";

  const pdaMatches =
    gatewayConfigDerived.equals(GATEWAY_CONFIG_PDA) &&
    guardianSetDerived.equals(GUARDIAN_SET_PDA) &&
    mintStateDerived.equals(MINT_STATE_PDA) &&
    recipientBalanceDerived.equals(RECIPIENT_BALANCE_PDA);

  const guardianKeypairsReady = guardianChecks.every(g => g.env_present && g.readable && g.pubkey_matches);
  const accountsReady = splMint.exists && recipientAta.exists && mintState.exists && recipientBalance.exists && gatewayConfig.exists && guardianSet.exists;

  Object.assign(result, {
    payer_check: payerCheck,
    guardian_checks: guardianChecks,
    guardian_keypairs_ready: guardianKeypairsReady,
    expected_guardians_present_onchain: expectedGuardiansOnchain,
    pda_matches_known_constants: pdaMatches,
    canonical_event_key_hex: CANONICAL_EVENT_KEY.toString("hex"),
    processed_event_pda: processedEventPda.toBase58(),
    processed_event_bump: processedEventBump,
    processed_event_exists_before: processedEvent.exists,
    signed_message_source: "expected_payload_hash",
    signed_message_preimage_schema: "sha256(domain || processed_event || route_id || mint || recipient_token_account || amount_u64_le || guardian_set_id)",
    signed_message_hex: signedMessage.toString("hex"),
    spl_mint_supply_before: readU64LE(splMint.data, 36),
    recipient_ata_amount_before: readU64LE(recipientAta.data, 64),
    mint_state_total_supply_before: readU128LE(mintState.data, 48),
    recipient_balance_amount_before: readU128LE(recipientBalance.data, 80),
    accounts_ready: accountsReady,
    prestate_zero: prestateZero,
    payload_account_meta_count: 11,
    real_account_count: 12,
    instructions_sysvar_account_index: 11,
    ready_for_option1_execution_builder: payerCheck.pubkey_matches_expected && guardianKeypairsReady && expectedGuardiansOnchain && pdaMatches && accountsReady && prestateZero,
  });

  const blockers = [];
  if (!payerCheck.pubkey_matches_expected) blockers.push("payer_keypair_not_expected_authority");
  if (!guardianKeypairsReady) blockers.push("guardian_keypair_env_missing_or_pubkey_mismatch");
  if (!expectedGuardiansOnchain) blockers.push("expected_guardians_not_present_onchain");
  if (!pdaMatches) blockers.push("pda_mismatch");
  if (!accountsReady) blockers.push("required_state_or_token_account_missing");
  if (!prestateZero) blockers.push("prestate_not_zero");
  result.blocker = blockers.length ? blockers.join(",") : "none";

  fs.mkdirSync(`${EVIDENCE_DIR}/results`, { recursive: true });
  fs.writeFileSync(`${EVIDENCE_DIR}/results/option1_preflight_result.json`, JSON.stringify(result, null, 2));

  console.log("=== option1 builder preflight compact summary for chat ===");
  for (const [k, v] of Object.entries(result)) printKV(k, v);
}

main().catch(err => {
  console.log("=== option1 builder preflight compact summary for chat ===");
  printKV("package", "runtime-state-provisioning-minimal-live-smoke-option1-builder-preflight-no-execution");
  printKV("preflight_failed", true);
  printKV("error", err && err.message ? err.message : String(err));
  printKV("transactions_executed", false);
  printKV("deploy_executed", false);
  printKV("upgrade_executed", false);
  printKV("push_executed", false);
  process.exitCode = 1;
});

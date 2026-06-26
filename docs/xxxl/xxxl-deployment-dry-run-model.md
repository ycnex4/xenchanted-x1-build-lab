# XXXL Deployment Dry-Run Model

## Purpose

This document defines the first deployment dry-run model for XXXL Program v1.

This is still planning / model work.

It is not production deployment code.

It does not use RPC.

It does not require secrets.

## Why this stage exists

Theo review listed deployment dry run as a missing production-readiness item.

This stage defines what must be proven before a future live deployment candidate can be considered ready.

## Dry-run mode

The only supported dry-run mode is:

    OFFLINE_ONLY

The dry run must not:

- use RPC
- attempt deployment
- load secret material
- expose private keys
- create manual mint paths
- create premine paths
- create founder allocation paths
- bypass gateway authorization

## Mandatory checks

The dry run must include:

- route policy validation
- incident policy validation
- account schema validation
- transition simulation
- Genesis supply invariant validation
- no manual mint path
- no premine
- no founder allocation
- no RPC usage
- no secrets
- authority freeze plan
- public disclosure readiness

## Forbidden capabilities

The dry-run policy must explicitly forbid:

- RPC usage
- live deployment
- secret material
- manual mint
- balance rewrite
- founder allocation
- premine
- upgrade bypass

## Required artifacts

The dry run must produce:

- parameter manifest
- test report
- supply invariant report
- incident runbook
- freeze plan
- public disclosure draft

## Successful dry-run report

A successful report requires:

- every mandatory check passed
- every check has evidence
- no forbidden capability detected
- every required artifact produced
- no RPC usage
- no deployment attempt
- no detected secrets

## Deployment meaning

This model does not authorize deployment.

It only defines what must be satisfied before moving to a live deployment-readiness stage.

## Non-goals

This stage does not implement:

- live deployment script
- RPC calls
- production deployment transaction
- live guardian keys
- secret handling

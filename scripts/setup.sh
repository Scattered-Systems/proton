#!/usr/bin/env bash
nix flake update
nix shell -c cargo build --workspace

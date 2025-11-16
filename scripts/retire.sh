#!/usr/bin/env bash

set -e

nm=$(basename "${1%.rs}")
printf "%s\n" "$nm"
mv "ice/${nm}.rs" ice/attic || true
mv "ice/${nm}.out" ice/attic || true
rm "ice/dup/${nm}.rs" ice/attic
rm "ice/dup/${nm}.out" ice/attic
sed -i "s|.*ice/${nm}.*||" src/ice.rs

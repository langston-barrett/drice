#!/usr/bin/env bash

set -euo pipefail

function bar {
  _progress=$(((${1}*100/${2}*100)/100))
  _done=$(((${_progress}*4)/10))
  _left=$((40-$_done))
  _fill=$(printf "%${_done}s")
  _empty=$(printf "%${_left}s")
  printf "\r[${_fill// /#}${_empty// /-}] ${_progress}%%"
}

files=$(ls ice/*.rs ice/dup/*.rs | wc -l)
done=0
for f in ice/*.rs ice/dup/*.rs; do
  bar ${done} ${files}

  msg="(error: internal compiler error:|error: the compiler unexpectedly panicked|rustc interrupted by SIGSEGV)"
  out=$(rustc +nightly --crate-type=lib "${f}" 2>&1 || true)
  echo "${out}" > "${f%.rs}.out"
  if echo "${out}" | grep -E "${msg}" 2>&1 > /dev/null; then
    # echo "ICE! ${f}"
    true
  else
    no=$(basename ${f%.rs})
    state=$(gh --repo rust-lang/rust issue view ${no} --json 'state' --jq '.state')
    if [[ ${state} == OPEN ]]; then
      printf "\nIssue open, but no ICE: %s\n" "${f}"
    elif [[ ${state} == CLOSED ]]; then
      printf "\nFixed: %s\n" "${f}"
      ./scripts/retire.sh "${f}"
    else
      printf "\Unknown state %s for %s\n" "${state}" "${f}"
      exit 1
    fi
  fi

  done=$((done+1))
done
printf '\n'

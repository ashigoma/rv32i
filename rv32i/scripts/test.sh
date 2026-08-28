#!/bin/bash
cd $(dirname $0)/..

ELF_PATH="$1"
BASE_PATH="${ELF_PATH%.elf}"
NAME=$(basename "$BASE_PATH")

SPIKE_TRACE="${BASE_PATH}.trace.sim.log"
SPIKE_LOG="${BASE_PATH}.sim.log"
SV_TRACE="${BASE_PATH}.trace.sv.log"
SV_LOG="${BASE_PATH}.sv.log"

./scripts/run_spike.sh "$ELF_PATH" "$SPIKE_TRACE" "$SPIKE_LOG"
./scripts/run_sv.sh "$ELF_PATH" "$SV_TRACE" "$SV_LOG"

if diff -q "$SPIKE_TRACE" "$SV_TRACE" > /dev/null; then
    echo -e "\033[32m[pass] $NAME\033[0m"
else
    echo -e "\033[31m[fail] $NAME\033[0m"
    echo "output diff:"
    diff -u "$SPIKE_TRACE" "$SV_TRACE" \
        | sed -E '/^(---|\+\+\+|@@)/d' \
        | sed -e "s/^\+.*/$(printf '\033[32m')&$(printf '\033[0m')/" \
              -e "s/^\-.*/$(printf '\033[31m')&$(printf '\033[0m')/" || true
    exit 1
fi
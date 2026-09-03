#!/bin/sh

CTX="${3:-3}"

awk -v ctx="$CTX" '
  NR == FNR {
    a[NR] = $0
    next
  }

  {
    if (a[FNR] != $0) {
      skip = FNR - ctx - 1
      if (skip > 0) printf "(%d lines)\n", skip
      for (i = FNR - ctx; i < FNR; i++) {
        if (i > 0) printf " %s\n", a[i]
      }
      printf "\033[31m-%s\033[0m\n", a[FNR]
      printf "\033[32m+%s\033[0m\n", $0
      mismatch = 1
      mismatch_line1 = FNR
      mismatch_line2 = FNR
      exit
    }
  }

  END {
    total2 = FNR
    total1 = length(a)

    if (!mismatch && total2 < total1) {
      skip = total2 - ctx
      if (skip > 0) printf "(%d lines)\n", skip
      for (i = total2 - ctx + 1; i <= total2; i++) {
        if (i > 0) printf " %s\n", a[i]
      }
      printf "\033[31m-%s\033[0m\n", a[total2 + 1]
      printf "\033[32m+(EOF)\033[0m\n"
      mismatch_line1 = total2 + 1
      mismatch_line2 = total2
    }

    rem1 = (total1 > mismatch_line1) ? total1 - mismatch_line1 : 0
    rem2 = (total2 > mismatch_line2) ? total2 - mismatch_line2 : 0

    if (rem1 > 0 || rem2 > 0) {
      printf "(\033[31m%d\033[0m / \033[32m%d\033[0m lines)\n", rem1, rem2
    }
  }
' "$1" "$2" || true
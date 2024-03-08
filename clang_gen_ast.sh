#!/usr/bin/env bash

set -u

clang++ -Xclang -ast-dump=json -fsyntax-only -nostdinc++ -nostdinc -fparse-all-comments $1

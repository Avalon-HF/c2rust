#!/usr/bin/env bash

set -u

clang++ -Xclang -ast-dump=json -fsyntax-only -fparse-all-comments $1

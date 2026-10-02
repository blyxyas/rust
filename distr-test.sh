#!/bin/bash

set -euxo


./x test "$@" --dry-run --tests | rg -o "\{.*\}" | tr -d ',' | tr -d '{' | tr -d "}" >> hola.txt

# if [[ $test_output == *"{}"* ]]; then

ssh rustvm 'x t'
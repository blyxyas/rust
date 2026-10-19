#!/bin/bash

changed_files=$(git status | rg "modified: .*" | rg "[^ ]*.rs" -o | sed -z 's/\n/ :\/home\/moon\/git\/rust-clone\/.\//g' )
changed_files=${changed_files::-29}

# to_test=$(./x test "$@" --dry-run --tests | rg -o "\{.*\}" | tr -d ',' | tr -d '{' | tr -d "}")
# to_test=$(echo "$to_test" | tr -d "rustc-main" | echo)

# ssh moon@192.168.1.148 "cd ~/git/rust/ && git restore . && rsync --relative --mkpath -vz moon@192.168.1.136:/home/moon/git/rust-clone/./${changed_files} ~/git/rust/"
ssh moon@192.168.1.148 "cd ~/git/rust/ && git restore . && rsync -ar --delete --relative --exclude 'build' --exclude '.git' --mkpath -vz moon@192.168.1.136:/home/moon/git/rust/./compiler :/home/moon/git/rust/./library :/home/moon/git/rust/./tests :/home/moon/git/rust/./bootstrap.toml :/home/moon/git/rust/./src ~/git/rust"

ssh moon@192.168.1.148 "cd ~/git/rust/ && ./x test $1 -j16 &> /var/log/distr-test.log"
./x test $2 &> /var/log/distr-test.log &

# tmux new-session \; send-keys 'tail -f /var/log/distr-test.log' C-m \; split-window -h \; send-keys 'ssh moon@192.168.1.148 "tail -f /var/log/distr-test.log"' C-m \;

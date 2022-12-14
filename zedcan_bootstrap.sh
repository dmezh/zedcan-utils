#!/bin/bash

# zedcan bootstrapping!
# We will:
# - add a nameserver (static for Cooper EE network)
# - install the rust toolchain
# - add goodies to home directory

echo "nameserver 199.98.27.171" >> /etc/resolv.conf
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cp -r home_dir/* ~

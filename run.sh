#!/bin/bash

export PANTS_CONSUMER_KEY=`jq -r .pants.PANTS_CONSUMER_KEY < ../server-setup/secrets/vars.json`
export PANTS_ACCESS_TOKEN=`jq -r .pants.PANTS_ACCESS_TOKEN  < ../server-setup/secrets/vars.json`

export PANTS_SITE="https://pants.dport.me"
export PANTS_STATIC_ROOT="/tmp/pants_static"
export PANTS_PAGES_ROOT="/tmp/pants_pages"

export ROCKET_PORT=8765
export ROCKET_ADDRESS=0.0.0.0

mkdir -p $PANTS_STATIC_ROOT
mkdir -p $PANTS_PAGES_ROOT

export RUST_LOG=info
export RUST_LOG_STYLE=always

cargo run

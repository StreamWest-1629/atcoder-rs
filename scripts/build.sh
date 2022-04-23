#!/bin/sh

cd $1
echo `pwd`
cargo build --bin atcoder_submit --package atcoder_submit
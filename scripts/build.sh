#!/bin/sh

cd $1
echo `pwd`
mold -run cargo build --bin atcoder_submit --package atcoder_submit
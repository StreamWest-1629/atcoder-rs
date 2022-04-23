#!/bin/sh

# example: ./scripts/create.sh abc/123/a

current_dir=`dirname $0`
contest_dir=$1

cd ./$current_dir/../
root_dir=`pwd`
mkdir -p $contest_dir

cp -r ${root_dir}/template $contest_dir

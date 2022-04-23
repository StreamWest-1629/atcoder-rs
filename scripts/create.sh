#!/bin/sh

# example: ./scripts/create.sh abc/123/a

current_dir=`dirname $0`
contest_dir=$1
contest_name=`echo $contest_dir | sed -e 's%/%-%g'`

cd ./$current_dir/../
root_dir=`pwd`
mkdir -p $contest_dir
cd ./$contest_dir

echo "current dir:  $current_dir"
echo "contest name: $contest_name"

cargo generate --init \
    --path ${root_dir}/template \
    --name atcoder_submit

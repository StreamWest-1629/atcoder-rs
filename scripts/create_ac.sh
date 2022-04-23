#!/bin/sh

contest_dir="atcoder/$1"
md="# ${contest_dir}\n\n"

cd `dirname $0`
chmod +x ./create.sh

for task in "A" "B" "C" "D" "E" "F" "G" "H"; do
./create.sh "${contest_dir}/${task}"
md="$md
## $task\n
- \n
\n
"
done

cd ../
echo $md > $contest_dir/readme.md

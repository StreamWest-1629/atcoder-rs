#!/bin/sh

contest_dir="atcoder/$1"
md="# ${contest_dir}\n\n
## Results\n
| Contents | Execute Results (URL) |\n
| :-: | :-- |\n
"
content=

cd `dirname $0`
chmod +x ./create.sh

for task in "A" "B" "C" "D" "E" "F" "G" "H"; do
./create.sh "${contest_dir}/${task}"
md="$md
| $task |  |\n"

content="$content
### $task\n
- \n
\n
"
done

md="$md\n\n

## Comments\n
$content"

cd ../
echo $md > $contest_dir/readme.md

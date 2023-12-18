#!/bin/fish

for dir in */
	pushd $dir
	cargo clean
	popd
end

echo "done"

#!/bin/bash
CUR_DIR="$(cd "$( dirname "${BASH_SOURCE[0]}"  )" && pwd  )"
echo $CUR_DIR
ln -svf $CUR_DIR/husty-hook $CUR_DIR/../.git/hooks/pre-commit
ln -svf $CUR_DIR/husty-hook $CUR_DIR/../.git/hooks/pre-push


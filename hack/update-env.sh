#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

huber_bin=.huber/bin

shells=(bashrc zshrc)
for s in "${shells[@]}"; do
  if [ -f "$HOME"/."$s" ] && ! [[ ":$PATH:" == *":$HOME/$huber_bin:"* ]]; then
    echo "export PATH=\$HOME/$huber_bin:\$PATH" >>"$HOME"/."$s"
  fi
done

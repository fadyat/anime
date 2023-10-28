#!/bin/bash

function release() {
  cargo build --release --quiet

  if [[ ! -d "$HOME/.anime" ]]; then
    mkdir "$HOME/.anime"
    echo "created ~/.anime"
  fi

  cp target/release/anime ~/.anime/anime
  echo "copied anime to ~/.anime/anime"

  cp config/config.yaml ~/.anime/config.yaml
  echo "copied config.yaml to ~/.anime/config.yaml"

  if ! grep -qF "$HOME/.anime/anime" ~/.zshrc; then
    echo "$HOME/.anime/anime" >> ~/.zshrc
    echo "added $HOME/.anime/anime to ~/.zshrc"
  fi

  echo "done"
}

release "$@"

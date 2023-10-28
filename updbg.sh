#!/bin/bash

# Usage: update-background.sh <image_path>
# Description: Updates the background image of the current iTerm2 window.
# The image path can be relative to the current working directory or absolute.

if [[ $# -ne 1 ]]; then
  echo "Usage: update-background.sh <image_path>"
  exit 1
fi

image_path="$1"
if [[ ! "$image_path" =~ ^/ ]]; then
  image_path="$(pwd)/$image_path"
fi

if [[ ! -f "$image_path" ]]; then
  echo "File not found: $image_path"
  exit 1
fi

osascript -e "
on run image_path
  tell application \"iTerm2\"
    tell current session of current window
      set background image to \"$image_path\"
    end tell
  end tell
end run
" "$image_path"

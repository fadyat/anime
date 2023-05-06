#!/bin/bash

# Usage: remove-bg.sh <image_path>|<image_url> <fuzz>[optional]
# Description: Removes the background image if the passed image.

fuzz=5

function download_from_web() {
  image_url="$1"
  image_name=$(basename "$image_path")
  curl "$image_url" --output "$image_name" --silent
  echo "downloaded $image_name"
}

function convert_to_png() {
  image_path="$1"
  image_name=$(basename "$image_path")
  name_only="${image_name%.*}"
  convert "$image_name" "$name_only.png"
  echo "converted $image_name to $name_only.png"
}

function validate() {
  if [[ $# -ne 1 && $# -ne 2 ]]; then
    echo "Usage: remove-bg.sh <image_path>|<image_url> <fuzz>[optional]"
    exit 1
  fi

  image_path="$1"
  if [[ $# -eq 2 ]]; then
    fuzz="$2"
  fi
}

validate "$@"

if [[ "$image_path" =~ "https://" ]]; then
  download_from_web "$image_path"
  image_path="$image_name"
fi

if [[ ! "$image_path" =~ ^/ ]]; then
  image_path="$(pwd)/$image_path"
fi

if [[ ! -f "$image_path" ]]; then
  echo "File not found: $image_path"
  exit 1
fi

if [[ ! "$image_path" =~ .png$ ]]; then
  convert_to_png "$image_path"
  image_path="$name_only.png"
fi

image_name=$(basename "$image_path")

magick "$image_path" -fuzz "$fuzz"% -fill none -draw "color 1,1 floodfill" no_bg_"$image_name"
echo "removed background from $image_name and saved as no_bg_$image_name"
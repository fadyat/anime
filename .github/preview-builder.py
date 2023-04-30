import argparse
import os
import typing


def get_all_images_path(
    root: str,
    extensions=('.webp', '.png', '.jpg', '.jpeg', '.gif', '.svg'),
):
    for path, _, files in os.walk(root):
        for file in files:
            if file.endswith(extensions):
                yield os.path.join(path, file)


def create_preview_content_markdown(
    images: typing.Generator[str, None, None],
):
    content = []

    for image in images:
        redirect_to_image = f'[{image}]({image})'
        image_preview = f'<img src="{image}" width="200" />'
        content.append(f'{redirect_to_image}\n\n{image_preview}')

    return '\n\n'.join(content)


def save_preview_content_markdown(
    content: str,
    path: str,
):
    with open(path, 'w') as f:
        f.write('## Available images\n\n')
        f.write(content)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', help='Root directory')

    args = parser.parse_args()
    save_preview_content_markdown(
        content=create_preview_content_markdown(
            images=get_all_images_path(args.root),
        ),
        path='PREVIEW.md',
    )


if __name__ == "__main__":
    main()

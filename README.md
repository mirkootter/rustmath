# rustmath
**Important:** This project is currently under heavy development and not production ready yet.

Simple and lightweight math formular renderer.

Features:
* OpenType Math support
* WASM compatible
* Renders PNG files, optionally with source included as metadata
* Library, embeddable into any rust application

## WebDemo
Try it live at https://mirkootter.github.io/math-demo

By default, the generated PNG files include the source as metadata. This leads to slightly larger
image files, but they can be edited: Just drag&drop them into the web demo.

## Sample images
**Note:** The following images are taken from the `ci/test-images` folder and are therefore tested
by the CI. Therefore, the CI ensures that they match the current version.

![](ci/test-images/cauchy.png)

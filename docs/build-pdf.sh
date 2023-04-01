#!/bin/sh
set -e

is_installed() {
    if ! command -v "$0" >> /dev/null
    then
        printf "\x1b[31;1m%s could not be found!\x1b[0m" "$0"
        exit
    fi
}


# NOTE: check if code is POSIX
TEX_CMD="pdflatex"

clean() {
    rm -f ./*.aux ./*.log ./*.gz ./*.toc ./*.dvi
}

main() {
    is_installed "${TEX_CMD}"
    # HACK: this is stupid, but it works soooo...
    clean
    ${TEX_CMD} thesis
    biber thesis
    ${TEX_CMD} thesis
    clean
}
main

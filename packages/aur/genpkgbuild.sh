#!/bin/bash

CARGOTOMLPATH=../../Cargo.toml
name=$(sed -n -E "s/name = \"(.+)\"/\1/p" $CARGOTOMLPATH)
version=$(grep -m 1 '^version = ' $CARGOTOMLPATH | cut -f 3 -d ' ' | tr -d \")
description=$(sed -n -E "s/description = \"(.+)\"/\1/p" $CARGOTOMLPATH)
license=$(sed -n -E "s/license = \"(.+)\"/\1/p" $CARGOTOMLPATH)
url=$(sed -n -E "s/repository = \"(.+)\"/\1/p" $CARGOTOMLPATH)
TARGZ="$name-v$version-x86_64-unknown-linux-musl.tar.gz"
TARGZURL="$url/releases/download/v$version/$TARGZ"
rm $TARGZ
wget $TARGZURL
sha256=$(sha256sum $TARGZ | cut -d ' ' -f1)
maintainer=$(sed -n -E "s/authors = \[\"(.+)\"\]/\1/p" $CARGOTOMLPATH)

cat >PKGBUILD <<EOL
# Maintainer: $maintainer
pkgname=$(name)-bin
pkgver=$version
pkgrel=1
pkgdesc="$description"
url="$url"
license=("$license")
arch=("x86_64")
provides=("$name")
source=("$TARGZURL")
sha256sums=("$sha256")

package() {
    install -Dm755 "$name-v$version-x86_64-unknown-linux-musl/$name" "\$pkgdir/usr/bin/$name"
}
EOL

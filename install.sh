#!/bin/bash

PREFIX=${PREFIX:='/usr/local'}
DATADIR=${DATADIR:=${PREFIX}/share}
BINDIR=${BINDIR:=${PREFIX}/bin}


# --- Compilation ------------------------------------------
echo 'Compiling...'
if !( cargo build --release ); then
    echo 'Compile failed.'
    exit 1
fi
echo 'Compilation complete.'


# --- Installation -----------------------------------------
echo 'Installing...'

if [[ -w ${PREFIX} ]]; then
    install -CDTv -m 755 "target/release/fir" "${BINDIR}/fir"
    install -CDTv -m 644 "data/fir.png" "${DATADIR}/icons/hicolor/48x48/apps/com.github.treecase.fir.png"
    install -CDTv -m 644 "data/fir.svg" "${DATADIR}/icons/hicolor/scalable/apps/com.github.treecase.fir.svg"
    install -CDTv -m 644 "data/fir.desktop" "${DATADIR}/applications/com.github.treecase.fir.desktop"
else
    sudo install -CDTv -m 755 "target/release/fir" "${BINDIR}/fir"
    sudo install -CDTv -m 644 "data/fir.png" "${DATADIR}/icons/hicolor/48x48/apps/com.github.treecase.fir.png"
    sudo install -CDTv -m 644 "data/fir.svg" "${DATADIR}/icons/hicolor/scalable/apps/com.github.treecase.fir.svg"
    sudo install -CDTv -m 644 "data/fir.desktop" "${DATADIR}/applications/com.github.treecase.fir.desktop"
fi

echo 'Installation complete.'

echo 'Done.'

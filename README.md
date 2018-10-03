# Skua

Skua is a library for automatically leaking out information from ELF binaries. Skua is strongly inspired from [pwntools dynelf](https://github.com/Gallopsled/pwntools/blob/dev/pwnlib/dynelf.py), though I hope to add additional automation. The end goal is 1) You provide primitives, 2) Skua puts them together automagically for exploitation.
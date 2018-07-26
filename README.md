oxi
==========

[![Travis](https://img.shields.io/travis/CalmBit/oxi.svg?style=flat-square)](https://travis-ci.org/CalmBit/oxi)


**oxi** is a client for **oxidation**, a bittorrent server, written in Rust. This is an extremely basic client,
that will be able to operate in two modes:

## Immediate
Immediate mode retains no client state - **oxi** starts, queries the server or gives it information,
returns whatever it recieves, and exits. It's more used for peeking and poking, instead of being a
monitoring or interactive application.

## Active
Active mode is stateful - an ncurses interface for continually interacting with an oxidation server. This
is more like a traditional torrent client, in the fact that you'll have a constant bead on the state of
each torrent, and be able to add more torrents or stop torrents actively.

merv
====

[![Build Status](https://travis-ci.org/mfs/merv.svg)](https://travis-ci.org/mfs/merv)

## Description

A simple utility for helping to solve crosswords. At the moment it is basically
a simplified grep though I have some ideas for extra functionality.

Merv expects to find a dictionary file at `/usr/share/dict/merv`. Create a
symbolic link to `/usr/share/dict/words` if it exists or drop in a suitable word
list. [Puzzlers' League][0] has links to suitable lists.

## Usage

Usage is simple. Pass the word you are looking for with blanks replaced with '.'

    $ merv .x..p.e # list matches
    example
    exciple
    $ merv listen # list anagrams
    enlist
    listen
    silent
    tinsel

[0]: http://www.puzzlers.org/dokuwiki/doku.php?id=solving:wordlists:about:start 

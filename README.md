# PalladiumOS (Formerly FrostDOS)

## An OS (more of a kernel) coded entirely in Rust

![Screenshot](Screenshot_20260608_213044.png)

## Warning
Currently compling using 'cargo build' is broken for some Goddamn reason so it only works on my pc I really dont know why.

## Notice
I do want to make things clear and say i used AI for some parts of my code as this is my first kernel so I dont understand much yet, I am so sorry and in the future i'll try to use less AI

i used AI for
* the filesystem
* the shell
* text editor (as a guide)

## Features
* Basic Shell
* Heap Allocation
* Keyboard Input
* Filesystem

## Instructions for Running the Kernel
* Install QEMU
* Download the Latest Release
* Run this Command: qemu-system-x86_64 -drive format=raw,file=[location of BIN file
* The system is running

## Commands
* echo - Echos back your Message
* about - Kernel info
* clear - Clear the screen
* help - Show the help message
* reboot - Reboots the system
* panic - Causes system panic
* Filesystem commands shown in screenshot
* edit <file> - Opens a file in the text editor

## Credits
Philipp Oppermann for his 'Writing an OS in Rust' guide

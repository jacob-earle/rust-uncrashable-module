#!/bin/bash
#This short script will install all of the necessary dependencies to compile this project
#A reboot will necessary in order to change the default kernel that will be downloaded
#Should be run on a fresh install of Ubuntu Bionic 18.04 without rust installed
sudo apt install curl llvm clang

#Install rustup in home directory
cd $HOME
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#Adding rustup to local environment
source $HOME/.cargo/env
echo 'export PATH=$PATH:$HOME/.cargo/bin' >> ./.bashrc
chmod +x ./.bashrc
source ./.bashrc

#installing necessary rust toolchains and components
rustup toolchain install nightly
rustup default nightly
rustup component add --toolchain=nightly rust-src rustfmt

echo "Before compiling, make sure you have installed the appropriate headers for your linux kernel version: `uname -r`.
Then run 
	make clean
and
	cargo clean
to clear all unnecessary files."

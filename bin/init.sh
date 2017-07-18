#!/bin/bash 
mkdir ~/Code-Fight
git clone https://github.com/bryanyang0528/Code-Fight.git ~/Code-Fight

sudo mkdir -m 777 /test
mkdir /test/$USER
cp -rf ~/Code-Fight /test/$USER/

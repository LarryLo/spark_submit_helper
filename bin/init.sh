#!/bin/bash 
mkdir ~/Code-Fight
git clone https://github.com/TaiwanSparkUserGroup/Code-Fight ~/Code-Fight

sudo mkdir -m 777 /test
mkdir /test/$USER
cp -rf ~/Code-Fight /test/$USER/

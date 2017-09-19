#!/bin/bash 
set -e

sudo mkdir -p /test/source/Code-Fight
sudo chown $USER:$USER -R /test
git clone https://github.com/bryanyang0528/Code-Fight.git /test/source/Code-Fight

#! /bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR
./testgame $1 $2 300000 2>&1 | tee logs/game-$(date +"%Y-%m-%d_%H-%m-%S").log
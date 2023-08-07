#!/usr/bin/env bash

if [ -e /etc/app/config/FirstTimeSetup ] 
then
    exec /bin/server --config /etc/app/config/config.json
else
    touch /etc/app/config/FirstTimeSetup
    exec /bin/server --config /etc/app/config/config.json --setup
fi
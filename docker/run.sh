#!/usr/bin/env bash

if [ -e /etc/app/config/vmids.json ] 
then
    exec /bin/server --config /etc/app/config/config.json
else
    exec /bin/server --config /etc/app/config/config.json --setup
fi

echo "Server is Going down!"
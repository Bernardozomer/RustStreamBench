#!/bin/bash

LICENSE_KEY=$1
FILE_NAME="GeoLite2-City"
LOG_URL="https://raw.githubusercontent.com/GMAP/DSPBench/master/dspbench-threads/data/http-server.log"
DB_URL="https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key=${LICENSE_KEY}&suffix=tar.gz"

mkdir inputs
curl --silent -o inputs/http-server.log $LOG_URL

if curl --silent -o inputs/${FILE_NAME}.tar.gz $DB_URL; then
	tar -xf inputs/${FILE_NAME}.mmdb.tar.gz -C inputs/
else
	echo "Failed to download database"
fi


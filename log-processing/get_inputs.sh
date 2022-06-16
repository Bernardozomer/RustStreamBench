#!/bin/bash

LICENSE_KEY=$1
FILE_NAME="GeoLite2-City"
URL="https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City&license_key=${LICENSE_KEY}&suffix=tar.gz"

if curl --silent -o inputs/${FILE_NAME}.tar.gz $URL; then
	tar -xf inputs/${FILE_NAME}.mmdb.tar.gz -C inputs/
else
	echo "Failed to download database"
fi


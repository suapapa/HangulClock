#/bin/bash

set -e

GOARCH=arm64 GOOS=linux go build
scp ./sbc_hangulclock orangepi@opi-hangulclock.local:~/
scp ./hangulclock.service orangepi@opi-hangulclock.local:~/

ssh orangepi@opi-hangulclock.local echo "sudo ln -s /home/orangepi/hangulclock.service /lib/systemd/system/"
#!/bin/sh

# Start syslog-ng to show logs
#/usr/sbin/syslog-ng -F --no-caps &

# Start dinit to run all services
exec /usr/sbin/dinit --container --services-dir /etc/dinit.d/
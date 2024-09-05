#!/usr/bin/env sh
rmmp -m ../modals/modals.txt -t ./ -p db > ./install.sql
rmmp -m ../modals/modals.txt -t ./ -p rust > ../src/modals.rs
echo "Done"

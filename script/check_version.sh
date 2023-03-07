#!/usr/bin/env bash

check_version=$1

lib_version=`grep version Cargo.toml | head -n 1`

if [ "$lib_version" == "version = \"$check_version\"" ]; then
    echo "Version is matched"
else
    echo "Version is not matched"
    exit 1
fi
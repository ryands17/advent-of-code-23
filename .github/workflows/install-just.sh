#!/bin/bash

if ! command -v just &> /dev/null
then
  cargo install just@1.16.0
fi

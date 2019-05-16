#!/bin/bash
set -eu

if [ ! -z ${CIRCLECI+x} ]; then
  # Git needs a user/email to commit under
  git config --global user.name "CI Deploy"
  git config --global user.email "ci@example.com"
fi

#!/usr/bin/env bash
PROJECT_ROOT="$( cd -- "$(dirname "$0")/.." >/dev/null 2>&1 ; pwd -P )"
for COOK_FILE in $PROJECT_ROOT/content/recipes/*.cook; do
  cook recipe -o  ${COOK_FILE/cook/md} -f markdown $COOK_FILE
done



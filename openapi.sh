#!/bin/bash

rm openapi -rf

docker run --rm -v "$PWD/openapi:/local" -v "$PWD/api.yaml:/api.yaml" openapitools/openapi-generator-cli generate \
                        -i "/api.yaml" \
                        -g rust \
                        -o /local

sudo chown `whoami`:`whoami` openapi -R
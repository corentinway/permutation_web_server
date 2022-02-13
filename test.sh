#!/bin/bash
curl -i -X POST localhost:8080 \
  -d {"input": [1, 2, 3], "max": 4} \
  -H Content-Type: application/json
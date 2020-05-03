#!/bin/bash

curl http://127.0.0.1:3000/ && echo

curl http://127.0.0.1:3000/hello && echo

curl http://127.0.0.1:3000?abc=100 && echo

curl http://127.0.0.1:3000/:aaa && echo


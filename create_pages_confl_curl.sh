#!/bin/bash

#for i in {1..5}
#do
#   echo "Welcome $i times"
#done


for a in {1..30}

do 

 curl -u admin:admin -X POST -H 'Content-Type: application/json' -d '{"type":"page","title":"new page ${a}", "space":{"key":"DEV"},"body":{"storage":{"value":"<p>This is <br/> a new page</p>","representation": "storage"}}}' http://confl-loadb-pxymvhygf6ct-1493255270.us-west-2.elb.amazonaws.com/rest/api/content



done

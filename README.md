# Query Elastic Logs Like a Boss
Query logs with a 300s window from 5000s ago on an interval of 1m with a value of "foo":
```
loga --host https://<some-host> --port 443 -c 10 -d 300 -s 5000 -m foo -i 1
```

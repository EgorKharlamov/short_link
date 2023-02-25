docker image **~485MB**
```bash
docker exec -it  short_link-api-1 bash
cd /usr/loca/bin
diesel setup && diesel migration run
```

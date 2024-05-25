docker buildx build -t iorp_core .
docker run --rm --name iorp_core -it -d iorp_core
mkdir bin
docker cp iorp_core:/app/iorp_core.so bin/iorp_core.so
docker stop iorp_core
docker image rm iorp_core
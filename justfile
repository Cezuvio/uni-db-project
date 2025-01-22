stop:
  @docker stop $(docker ps -aq)

clean: stop
  @docker rmi -f $(docker images -q)

run:
  @docker compose up -d --build

docker-build:
  docker build -t frontier_indexer:latest .

docker-save:
  docker save -o ./frontier_indexer.tar frontier_indexer:latest


run-container:
	docker compose -f ./docker/docker-compose.yml up -d

down-container:
	docker compose -f ./docker/docker-compose.yml down
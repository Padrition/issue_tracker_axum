COMPOSE_FILE = docker-compose.yaml

up:
	sudo docker compose -f ${COMPOSE_FILE} up -d --build --remove-orphans

down:
	sudo docker compose -f ${COMPOSE_FILE} down
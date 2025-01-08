COMPOSE_FILE = docker-compose.yaml

run:
	sudo docker compose -f ${COMPOSE_FILE} up -d --build --remove-orphans

stop:
	sudo docker compose -f ${COMPOSE_FILE} down
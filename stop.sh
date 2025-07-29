#!/bin/bash

set -e

echo "Stopping Qdrant..."

if [ ! -f "docker-compose.yml" ]; then
    echo "❌ docker-compose.yml not found. Nothing to stop."
    exit 1
fi

if command -v docker-compose &> /dev/null; then
    docker-compose down
elif docker compose version &> /dev/null; then
    docker compose down
else
    echo "❌ Docker Compose not found"
    exit 1
fi

echo "Qdrant stopped successfully"

echo ""
echo "Container status:"
docker ps -a --filter "name=qdrant" --format "table {{.Names}}\t{{.Status}}"

#!/bin/bash

set -e

echo "Starting Qdrant..."

if [ ! -f "docker-compose.yml" ]; then
    echo "❌ docker-compose.yml not found. Run ./setup.sh first."
    exit 1
fi

if command -v docker-compose &> /dev/null; then
    docker-compose up -d
elif docker compose version &> /dev/null; then
    docker compose up -d
else
    echo "❌ Docker Compose not found"
    exit 1
fi

echo "Waiting for Qdrant to start..."
sleep 3

# Check if Qdrant is running
for i in {1..30}; do
    if curl -s http://localhost:6333/health > /dev/null 2>&1; then
        echo "Qdrant is running!"
        echo "REST API: http://localhost:6333"
        echo "gRPC API: localhost:6334"
        echo "Web UI: http://localhost:6333/dashboard"
        exit 0
    fi
    echo "Still waiting... ($i/30)"
    sleep 5
done

echo "❌ Qdrant failed to start or is not responding"
echo "Check logs with: docker logs qdrant"
exit 1

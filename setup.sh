#!/bin/bash

set -e

echo "Setting up Qdrant environment..."

# Detect OS
OS="unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    OS="windows"
fi

echo "Detected OS: $OS"

# install Docker on Ubuntu
install_docker_ubuntu() {
    echo " Installing Docker on Ubuntu..."
    sudo apt-get update
    sudo apt-get install -y apt-transport-https ca-certificates curl gnupg lsb-release

    # add Docker's official GPG key
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

    # add Docker repository
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

    # install Docker
    sudo apt-get update
    sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

    # add user to docker group
    sudo usermod -aG docker $USER
    echo "You may need to log out and back in for Docker group permissions to take effect"
}

# install Docker on CentOS/RHEL/Fedora
install_docker_centos() {
    echo "Installing Docker on CentOS/RHEL/Fedora..."
    sudo yum install -y yum-utils
    sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo
    sudo yum install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
    sudo systemctl start docker
    sudo systemctl enable docker
    sudo usermod -aG docker $USER
    echo "You may need to log out and back in for Docker group permissions to take effect"
}

# install Docker if not present, supporting os
if ! command -v docker &> /dev/null; then
    echo "Docker not found. Installing Docker..."

    if [[ "$OS" == "linux" ]]; then
        if command -v apt-get &> /dev/null; then
            install_docker_ubuntu
        elif command -v yum &> /dev/null; then
            install_docker_centos
        else
            echo "Unsupported Linux distribution. Please install Docker manually."
            echo "Visit: https://docs.docker.com/get-docker/"
            exit 1
        fi
    elif [[ "$OS" == "macos" ]]; then
        echo "Please install Docker Desktop for Mac manually."
        echo "Visit: https://docs.docker.com/desktop/install/mac-install/"
        exit 1
    else
        echo "Unsupported OS. Please install Docker manually."
        echo "Visit: https://docs.docker.com/get-docker/"
        exit 1
    fi
else
    echo "✅ Docker is already installed"
fi

# docker compose
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null 2>&1; then
    echo "Docker Compose is not available. Please install Docker Compose."
    echo "Visit: https://docs.docker.com/compose/install/"
    exit 1
else
    echo "✅ Docker Compose is available"
fi

# rust
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    echo "Rust installed successfully"
else
    echo "Rust is already installed"
fi

# curl, pkg-config, libssl-dev
if [[ "$OS" == "linux" ]]; then
    if command -v apt-get &> /dev/null; then
        echo "Installing additional dependencies..."
        sudo apt-get update
        sudo apt-get install -y curl jq build-essential pkg-config libssl-dev
    elif command -v yum &> /dev/null; then
        echo "Installing additional dependencies..."
        sudo yum install -y curl jq gcc openssl-devel pkg-config
    fi
fi

# qdrant directories
echo "Creating directories..."
mkdir -p ./qdrant_data
mkdir -p ./logs

# verify docker-compose.yml exists
if [ ! -f "docker-compose.yml" ]; then
    echo "❌ docker-compose.yml not found in current directory"
    exit 1
fi

# pull Qdrant image
echo "Pulling Qdrant Docker image..."
docker pull qdrant/qdrant:latest

# start Docker service if not running (Linux only)
if [[ "$OS" == "linux" ]]; then
    if ! systemctl is-active --quiet docker; then
        echo "🔧 Starting Docker service..."
        sudo systemctl start docker
    fi
fi

echo ""
echo "Setup complete!"
echo ""
echo "Next steps:"
echo "  run ./run.sh to start Qdrant"
echo "  run ./stop.sh to stop Qdrant"
echo ""
if groups $USER | grep -q docker; then
    echo "User is in docker group"
else
    echo "You may need to log out and back in for Docker group permissions to take effect"
fi

+++
title = "Docker Basics for Beginners"
date = "2025-01-01"
update = "2025-01-03"
summary = "Introduction to containerization with Docker"
path = "docker-basics-beginners.md"
+++

# Docker Basics for Beginners

Docker is a platform that allows you to develop, deploy, and run applications in containers. Containers package an application and its dependencies together, ensuring consistency across different environments.

## Key Concepts

### Images and Containers

A Docker image is a lightweight, stand-alone, executable package that includes everything needed to run a piece of software. A container is a runtime instance of an image.

### Dockerfile

A Dockerfile is a text document that contains all the commands a user could call on the command line to assemble an image. Using docker build can create an automated build that executes several command-line instructions.

## Common Commands

```bash
# Build an image from a Dockerfile
docker build -t my-app .

# Run a container from an image
docker run -d -p 8080:80 my-app

# List running containers
docker ps
```
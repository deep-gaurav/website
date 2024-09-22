---
title: "SelfCloud"
cover: "/assets/images/projects/selfcloud/cover.png"
tagline: "Your Personal, Self-Hosted PaaS Solution"
short_description: "SelfCloud is a fullstack Rust application built with Leptos, offering a self-hosted alternative to services like Heroku. It allows you to easily deploy, manage, and monitor your containerized applications with custom domain support and SSL management."
stack: Rust, Leptos, Docker, SSL
frontend_source: https://github.com/deep-gaurav/self-cloud
screenshots:
  - "/assets/images/projects/selfcloud/sc1.png"
  - "/assets/images/projects/selfcloud/sc2.png"
  - "/assets/images/projects/selfcloud/sc3.png"
  - "/assets/images/projects/selfcloud/sc4.png"
  - "/assets/images/projects/selfcloud/sc5.png"
  - "/assets/images/projects/selfcloud/sc6.png"
  - "/assets/images/projects/selfcloud/sc7.png"

priority: 80

---

## Overview

SelfCloud empowers developers to create their own Platform as a Service (PaaS) environment. Built entirely in Rust using the Leptos framework, it provides a robust, efficient, and user-friendly solution for deploying and managing containerized applications.

## Features

- **Project Management**: Create and manage multiple projects within your SelfCloud instance.
- **Custom Domain Support**: Assign custom domains to your projects with ease.
- **Automated SSL Management**: SelfCloud handles SSL certificate generation and renewal automatically.
- **Container Deployment**: Push and deploy Docker containers to your projects using generated tokens.
- **Port Exposure**: Automatically exposes configured ports to assigned domains.
- **Support Containers**: Deploy additional containers like databases to support your main application.
- **Monitoring and Logs**: Keep track of your application's performance and logs in real-time.
- **Terminal Access**: Attach a terminal to your containers for direct interaction and debugging.

## How It Works

1. Create a new project in SelfCloud and assign it a domain.
2. Generate an authentication token for the project.
3. Use the token to push your Docker container to SelfCloud.
4. SelfCloud deploys your container and exposes the configured port to the assigned domain.
5. Monitor your application's performance, view logs, and access the terminal as needed.

## Technical Details

- **Backend & Frontend**: Built with Rust using the Leptos framework for a unified, full-stack development experience.
- **Containerization**: Utilizes Docker for application deployment and isolation.
- **SSL Management**: Integrates with Let's Encrypt for automated SSL certificate provisioning and renewal.
- **Reverse Proxy**: Employs a reverse proxy to route traffic to the appropriate containers based on domain configuration.

## Key Advantages

- **Self-Hosted**: Maintain full control over your deployment environment and data.
- **Cost-Effective**: Eliminate ongoing PaaS subscription costs by hosting on your own infrastructure.
- **Customizable**: Tailor the platform to your specific needs and workflows.
- **Secure**: Benefit from Rust's security features and automated SSL management.
- **Performance**: Leverage Rust's speed and efficiency for optimal resource utilization.

## Use Cases

- **Personal Projects**: Deploy and manage your side projects with ease.
- **Small Teams**: Provide a centralized deployment solution for team projects.
- **Education**: Set up a controlled environment for teaching deployment and DevOps concepts.
- **Prototyping**: Quickly deploy and iterate on new ideas without complex setup.

Experience the power and flexibility of your own personal PaaS with SelfCloud - where control meets convenience in application deployment!
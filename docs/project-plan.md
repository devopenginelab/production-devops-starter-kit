# Project Plan: Production DevOps Starter Kit

## 1. Project Goal
Build a reusable, production-grade DevOps template that demonstrates how to securely containerize a backend microservice, enforce automated quality and security checks via CI/CD, and provision isolated AWS infrastructure using Infrastructure as Code (IaC).

## 2. Target Client Problem
Early-stage startups and small engineering teams often suffer from fragile manual deployments ("it works on my machine"), bloated Docker containers running with unsafe root privileges, and zero automated security checks before code reaches production. This starter kit provides a clean, repeatable blueprint to eliminate those bottlenecks.

## 3. Architecture & Technology Stack
- **Application (`app/`):** Lightweight Rust (Axum) HTTP microservice exposing `/health` and `/version` endpoints for container orchestration and load balancer health checks.
- **Containerization (`docker/`):** Multi-stage Dockerfile utilizing a minimal runtime image and a non-root user to minimize attack surface and image size.
- **CI/CD & Security (`.github/workflows/`):** GitHub Actions pipelines running code formatting, linting, unit tests, dependency vulnerability audits, and Git history secret scanning.
- **Infrastructure as Code (`terraform/`):** Modular Terraform configurations provisioning a custom AWS VPC (public/private subnets) and container compute resources.

## 4. Project Epics & Milestones
- **Epic 1: Governance & Documentation Foundation** — Repository structure, `.gitignore` security rules, project plan, and live status tracking.
- **Epic 2: Microservice & Hardened Containerization** — Minimal Rust API + multi-stage non-root Dockerfile + local Docker Compose testing.
- **Epic 3: Automated CI & Security Gates** — GitHub Actions workflows for build verification, linting, dependency auditing, and secret scanning.
- **Epic 4: AWS Infrastructure as Code (Terraform)** — Reusable VPC networking and compute definitions with remote state best practices.
- **Epic 5: Operational Runbooks & Portfolio Proof** — Architecture diagrams, Architectural Decision Records (ADRs), troubleshooting guides, and rollback procedures.

## 5. Constraints & Security Rules
- **Zero Proprietary Code:** 100% original, open-source configurations with no third-party or employer intellectual property.
- **Least Privilege:** Containers must never run as `root`; IAM and database roles must follow strict least-privilege boundaries.
- **Zero Committed Secrets:** All credentials must be injected at runtime via environment variables or secret managers.
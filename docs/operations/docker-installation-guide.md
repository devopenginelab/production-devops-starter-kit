This guide encapsulates industry best practices: installing from official vendor repositories (instead of outdated default OS packages), ensuring cryptographic verification via GPG keys, configuring secure non-root permissions, and documenting failure recovery steps.
Markdown

# Operational Runbook: Production Docker Installation

## 1. Objective
To install and configure Docker Engine, Docker CLI, and container orchestration plugins securely and reproducibly on a Linux production or development instance (Ubuntu/Debian).

## 2. Why Install from Official Repositories?
Operating system package managers (like Ubuntu's `apt`) often bundle older, long-term maintenance versions of Docker. Installing directly from Docker’s official repository ensures access to recent security patches, bug fixes, and modern container features (such as enhanced Buildx and Compose plugins).

---

## 3. Step-by-Step Installation Procedure

### Step 1: Remove Conflicting Packages
Ensure no legacy or third-party container packages interfere with the new installation:
```bash
sudo apt-get remove -y docker docker-engine docker.io containerd runc 2>/dev/null || true

Step 2: Install Prerequisite Utilities

Install packages required to handle secure HTTPS downloads and cryptographic keys:
Bash

sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg

Step 3: Add Docker's Official GPG Security Key

A GPG key ("GNU Privacy Guard key") is a cryptographic signature used to verify that the software packages you download genuinely originate from Docker and have not been tampered with.
Bash

sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL [https://download.docker.com/linux/ubuntu/gpg](https://download.docker.com/linux/ubuntu/gpg) | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

Step 4: Configure the Official Stable Repository

Create the repository source file.

    ⚠️ Production Gotcha: Always ensure the repository definition is written on a single, continuous line. Splitting deb [...] URLs across multiple lines with accidental newlines will corrupt the file and trigger Malformed entry errors in apt.

Bash

echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] [https://download.docker.com/linux/ubuntu](https://download.docker.com/linux/ubuntu) $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

Step 5: Install Docker Engine & Plugins

Update your package index and install the complete container toolset:
Bash

sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

4. Security Best Practice: Non-Root Execution

By default, the Docker daemon listens on a root-owned Unix socket, requiring sudo for every command. For operational efficiency and security best practices:

    Add your user to the docker security group:
    Bash

    sudo usermod -aG docker $USER

    Refresh your terminal group membership immediately:
    Bash

    newgrp docker

5. Verification & Health Check

Verify that the Docker daemon is active and running correctly without sudo:
Bash

docker run hello-world

Expected Outcome: You should see the official Docker welcome message explaining that your installation is working correctly.
6. Troubleshooting Common Failures
Symptom: Malformed entry 1 in list file /etc/apt/sources.list.d/docker.list

    Root Cause: A newline character accidentally broke the repository URL string when writing the source list.

    Fix:
    Bash

    sudo rm -f /etc/apt/sources.list.d/docker.list
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] [https://download.docker.com/linux/ubuntu](https://download.docker.com/linux/ubuntu) $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    sudo apt-get update

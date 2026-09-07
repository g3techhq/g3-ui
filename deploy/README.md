# Playground deployment

The g3-ui playground is a static Dioxus web application served by an unprivileged
Nginx container. GitHub Actions publishes two image tags to GitHub Container
Registry after a successful build on `main`:

- `ghcr.io/g3techhq/g3-ui-playground:latest`
- `ghcr.io/g3techhq/g3-ui-playground:sha-<commit>`

## One-time GitHub setup

1. Merge the deployment workflow and let its first `main` run finish.
2. In the GitHub organization, open **Packages**, then **g3-ui-playground**.
3. Open **Package settings** and change the package visibility to **Public**.

Public GHCR packages can be pulled without credentials. Package visibility is
separate from repository visibility, so verify this once after the first image
is published.

## Portainer stack

1. In Portainer, open the target environment and select **Stacks** → **Add stack**.
2. Give the stack a name such as `g3-ui-playground`.
3. Choose **Web editor** and paste the contents of
   [`compose.yaml`](compose.yaml), or choose **Git repository** and point
   Portainer at this public repository with `deploy/compose.yaml` as the
   Compose path.
4. Optionally set `G3_UI_PLAYGROUND_PORT` in the stack environment variables.
   It defaults to host port `8080`.
5. Deploy the stack.

The container starts automatically after Docker restarts and is restarted after
unexpected exits. `pull_policy: always` checks for a newer `latest` image
when the stack is redeployed or the container is recreated.

For unattended releases, use Portainer's stack webhook or another updater to
trigger a stack redeploy after GitHub publishes a new image. A restart policy
does not, by itself, replace a running container when a new image is pushed.

## Reverse proxy

Keep port `8080` private to the LAN when a reverse proxy such as Caddy,
Traefik, or Nginx Proxy Manager fronts the service. Route the chosen public
hostname to `http://<docker-host>:8080` and terminate TLS at the proxy.

## Local production check

```powershell
docker build --tag g3-ui-playground:local .
docker run --rm --publish 8080:8080 g3-ui-playground:local
```

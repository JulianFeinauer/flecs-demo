To build multi-plattform image the following result helped me: https://github.com/docker/buildx/issues/495#issuecomment-761562905

Then run:
```
docker buildx build --platform=linux/amd64,linux/arm64 -t jfeinauer/flecs-agent:demo .
```

on a ubuntu host.

# actix-connection-info-test

Simple Actix Web application that shows `HttpRequest.connection_info` return in
the browser. May help verifying that your reverse proxy is set up correctly.

### Building

In order for this application to be built, you'll need Docker. After you've
installed it, you just need to clone this repository and build image locally:

```bash
docker build -t actix-connection-info-test .
```

After image is built, you can run it: `docker run -p 8080:8080 actix-connection-info-test`.
After that, application is available at [http://localhost:8080](http://localhost:8080).

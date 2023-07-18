# AUTD3 User's Guide

# Build with docker

```
docker build -f ./Dockerfile . -t autd3-book:1.0
docker run -it --rm -d -e MDBOOK_BOOK__src=src/en --name autd3-book -p 3300:3000 --hostname="mdbook" autd3-book:1.0 mdbook serve -p 3000 -n mdbook
```

Then, open in [localhost:3300](http://localhost:3300/)

- If you want to build a Japanese version, run
    ```
    docker run -it --rm -d -e MDBOOK_BOOK__src=src/jp --name autd3-book -p 3300:3000 --hostname="mdbook" autd3-book:1.0 mdbook serve -p 3000 -n mdbook
    ```

# Author

Shun Suzuki, 2022-2023

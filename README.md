[![Build Status](https://travis-ci.org/yanns/zeugnis.svg?branch=master)](https://travis-ci.org/yanns/zeugnis)

# Development

## Libraries

- [iron](http://ironframework.io/), web framework
- [maud](https://www.gitbook.com/book/lfairy/maud/details), compile-time template engine

## Using rust nightly

Maud needs rust nightly for the moment.

```
rustup run nightly cargo test
```

```
rustup run nightly cargo run
```


# Deployment
## Heroku

Deployment to [heroku](https://zeugnis.herokuapp.com/) was configured with:

```
heroku create zeugnis --region eu --buildpack https://github.com/emk/heroku-buildpack-rust.git
git push heroku master
```


## Docker

Resources:

- [Docker & Rust: Statically Linking Binaries for Secure Execution in Untrusted Containers](http://betacs.pro/blog/2016/07/07/docker-and-rust/)
- [Docker environment for building musl based static rust binaries](https://github.com/clux/muslrust)

### Build a statically linked binary

#### On linux:
```
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release
```

#### Other (mac):
```
docker run -v $PWD:/volume -w /volume -t clux/muslrust cargo build --release
```

Executable: ./target/x86_64-unknown-linux-musl/release/zeugnis

### Create the docker container
```
docker build -t zeugnis .
```

### Test the docker container
```
docker run -p 8080:8080 --rm -t zeugnis --name zeugnis
```

```
docker ps -s
CONTAINER ID        IMAGE                 COMMAND                  CREATED             STATUS              PORTS                              NAMES                           SIZE
ce7f51a68325        zeugnis               "/zeugnis --name zeug"   4 seconds ago       Up 3 seconds        0.0.0.0:8080->8080/tcp             silly_ramanujan                 0 B (virtual 2.268 MB)
```

The application is accessible on [localhost:8080](http://localhost:8080)
or with docker-machine:
```
m=`docker-machine active`
ip=`docker-machine ip $m`
url=http://$ip:8080
echo $url
curl $url
```

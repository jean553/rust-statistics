# rust-statistics

## Start the container

```bash
vagrant up
```

## Connect to the container

```bash
vagrant ssh
```

## Run tests

```bash
cargo test
```

## Generate coverage

```bash
kcov --verify target/cov target/debug/{library-binary-file}
```

Check the output into `target/cov/index.html`.

## Generate documentation

```bash
cargo rustdoc -- --no-defaults
```

# Tasks for Ballpoint

## build

> Build the project

~~~pwsh
cargo build
~~~

## test

> Run tests

### test watch
~~~pwsh
cargo watch -x 'test -q --lib'
~~~

### test all

> Runs all the tests

~~~pwsh
cargo test
~~~

### test spec (spec)

> Runs a specific spec

~~~powershell
Write-Output "${spec}"
cargo test ${spec}
~~~

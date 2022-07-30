## Execution

The plasma simulations can be run with the following commands:

```
docker build -t plasma .
docker run -it --rm -v $(pwd):/usr/src/plasma-simulation --name plasma plasma
```

Or by executing the `run.sh` script, if you are in a Unix based environment.


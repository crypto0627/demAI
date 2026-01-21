set dotenv-load

## frontend



## backend
# start rust local backend
local-dev:
    cd apps/backend && cargo run mono

# clean target
local-clean:
    cd apps/backend && cargo clean
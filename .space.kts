job("Docker: Build and publish") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    host("Build artifacts and a Docker image") {
        env["HUB_USER"] = Secrets("dockerhub_username")
        env["HUB_TOKEN"] = Secrets("dockerhub_token")

        shellScript {
            content = """
                docker login --username ${'$'}HUB_USER --password "${'$'}HUB_TOKEN"
            """
        }

        dockerBuildPush {
            context = "."
            file = "Dockerfile"
            labels["vendor"] = "Scattered-Systems, LLC"
            tags {
                +"scsys/proton:backend"
                +"scsys/proton:backend_v0.1.${"$"}JB_SPACE_EXECUTION_NUMBER"
            }
        }
    }
}

job("(Disarray) Rust: Build and test workspace") {
    startOn {
        gitPush { 
            branchFilter {
                +"refs/heads/main"
                +"refs/tags/v*.*.*"
            }
        }
        schedule { cron("0 8 * * *") }
    }
    container(displayName = "Rust", image = "rust") {
        env["CARGO_REGISTRY_TOKEN"] = Secrets("cargo_registry_token")
        shellScript {
            interpreter = "/bin/bash"
            content = """
                apt-get update -y && apt-get upgrade -y
                apt-get install -y protobuf-compiler
                rustup default nightly && rustup target add wasm32-unknown-unknown --toolchain nightly
                cargo login ${'$'}CARGO_REGISTRY_TOKEN
                cargo test --all --all-features
            """
        }
    }
}
# reteam

> a small tool for updating github repository "team" topics

# 🤸 usage

```sh
$ cargo run -q -- --help
reteam 0.1.0
tool for managing updates to team owned github repositories

USAGE:
    reteam <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help            Prints this message or the help of the given subcommand(s)
    repos           List repos with a team tag
    update-topic    Update github repository team topic
```

## repos

List all github repositories under a organization tagged with a team name topic

```sh
$ GITHUB_TOKEN=xxx cargo run -q -- \
  repos \
    --organization github-org \
    --team team-name
```

## update-topic

Updates app github repositories under a organization tagged with a team name topic


```sh
$ GITHUB_TOKEN=xxx cargo run -q -- \
  update-topic \
  --organization github-org \
  --team team-name \
  --new-team new-team-name
```

# 👩‍🏭 development

This is a [rustlang](https://www.rust-lang.org/en-US/) application.
Go grab yourself a copy with [rustup](https://rustup.rs/).
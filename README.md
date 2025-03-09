# aws-logs-tui

## Goals

Using the AWS Console to quickly find, browse, and view Lambda logs is annoying.
Using the AWS CLI is just as bad, it requires wrapping multiple commands in a script.

I should be able to quickly find the relevant logs for my Lambda functions, browse them, and view the log entries I want.
Then I can export those log entries, or if I know what I'm looking for I can directly export them with a single command and not have to browse first.

AWS Lambda is the current entrypoint to find logs, but this tool is really an "AWS Service -> Cloudwatch Logs" browser which means any CW logs could be searched, browsed, viewed, and exported if you skipped requiring an AWS Service as an entrypoint...

Thus, the goals are:

1. A TUI for AWS Cloudwatch Logs
1. Automatic lookup of CW Logs from your AWS Service
1. Only support read-only operations

## Out of Scope

1. A TUI replacement for the AWS Console or AWS CLI
1. Management (write) operations

## Features

- Add `clap` for arg parsing, provide a help output and description
- Use args to dump logs directly to `STDOUT` if user knows what they want
- Dump in JSON (jsonline?) & "logfile" formats
- Add color theme support with [`tui-theme-builder`](https://github.com/preiter93/tui-theme-builder?tab=readme-ov-file)
- Load "most recent" logs regardless of age, don't make me try different age ranges
- Tail the logs for new Lambda invocations
- Generate AWS CLI (and console?) "links" to the current logs being viewed
- Use `~/.aws/configuration` for Profile selection, or environment, or commandline args
- Document required IAM policy permissions for Lambda & CW Logs

## TODO

- [x] Use [clap](https://docs.rs/clap/latest/clap/) for arg-parsing
- `aws_config`
  - [x] Use `AWS_PROFILE` from environment
  - [x] Use `--profile` from args
  - [x] Use `--region` from args
  - [ ] Refactor to be pretty & tested
  - [ ] Research other AWS TUIs/CLIs to see what they call `--profile/--region`
  - [ ] Create a TUI profile selector using `~/.aws/config` contents
- `aws_sdk_lambda`
  - [x] Print functions to STDOUT
  - [ ] Refactor to be pretty & tested
- [ ] Use [tui-realm](https://github.com/veeso/tui-realm) for MVC framework

## Issues

None yet...

## Other Rust TUIs for AWS

- [`rust-aws-tui`](https://github.com/resola-ai/rust-aws-tui)
- [`stu`](https://github.com/lusingander/stu?ref=terminaltrove)
- [`ssm-tui`](https://github.com/sandeshgrangdan/ssm-tui)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

# OpenAI EfficiencyCode Python SDK

Build Python applications that start EfficiencyCode threads, run turns, stream progress,
and control workspace access.

## Install

Install the SDK:

```bash
pip install openai-EfficiencyCode
```

## Quickstart

The SDK reuses your existing EfficiencyCode authentication when one is already
available:

```python
from openai_EfficiencyCode import EfficiencyCode

with EfficiencyCode() as EfficiencyCode:
    thread = EfficiencyCode.thread_start()
    result = thread.run("Explain this repository in three bullets.")
    print(result.final_response)
```

`thread.run(...)` returns a `TurnResult` containing the final response,
collected items, and token usage.

## Authentication

Existing EfficiencyCode authentication is reused automatically. To start ChatGPT
browser login explicitly:

```python
from openai_EfficiencyCode import EfficiencyCode

with EfficiencyCode() as EfficiencyCode:
    login = EfficiencyCode.login_chatgpt()
    print(login.auth_url)
    print(login.wait().success)
```

For device-code login:

```python
with EfficiencyCode() as EfficiencyCode:
    login = EfficiencyCode.login_chatgpt_device_code()
    print(login.verification_url, login.user_code)
    login.wait()
```

For API-key login:

```python
with EfficiencyCode() as EfficiencyCode:
    EfficiencyCode.login_api_key("sk-...")
```

## Built-In Help

Use Python's standard `help(openai_EfficiencyCode)`, `help(EfficiencyCode)`, or
`python -m pydoc openai_EfficiencyCode` documentation tools.

## Documentation

- [Getting started](https://github.com/openai/EfficiencyCode/blob/main/sdk/python/docs/getting-started.md)
- [API reference](https://github.com/openai/EfficiencyCode/blob/main/sdk/python/docs/api-reference.md)
- [FAQ](https://github.com/openai/EfficiencyCode/blob/main/sdk/python/docs/faq.md)
- [Examples](https://github.com/openai/EfficiencyCode/blob/main/sdk/python/examples/README.md)

The package is licensed under the
[repository Apache License 2.0](https://github.com/openai/EfficiencyCode/blob/main/LICENSE).

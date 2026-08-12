"""Python SDK for running EfficiencyCode workflows.

Start with :class:`EfficiencyCode` for synchronous applications or
:class:`AsyncEfficiencyCode` for async applications. Most programs create a thread and
run a turn::

    from openai_EfficiencyCode import EfficiencyCode, Sandbox

    with EfficiencyCode() as EfficiencyCode:
        thread = EfficiencyCode.thread_start(sandbox=Sandbox.workspace_write)
        result = thread.run("Describe this project.")
        print(result.final_response)
"""

from ._version import __version__
from .api import (
    ApprovalMode,
    AsyncChatgptLoginHandle,
    AsyncEfficiencyCode,
    AsyncDeviceCodeLoginHandle,
    AsyncThread,
    AsyncTurnHandle,
    ChatgptLoginHandle,
    EfficiencyCode,
    DeviceCodeLoginHandle,
    ImageInput,
    Input,
    InputItem,
    LocalImageInput,
    MentionInput,
    RunInput,
    Sandbox,
    SkillInput,
    TextInput,
    Thread,
    TurnHandle,
    TurnResult,
)
from .client import EfficiencyCodeConfig
from .errors import (
    EfficiencyCodeError,
    EfficiencyCodeRpcError,
    InternalRpcError,
    InvalidParamsError,
    InvalidRequestError,
    JsonRpcError,
    MethodNotFoundError,
    ParseError,
    RetryLimitExceededError,
    ServerBusyError,
    TransportClosedError,
    is_retryable_error,
)
from .retry import retry_on_overload

__all__ = [
    "__version__",
    "EfficiencyCodeConfig",
    "EfficiencyCode",
    "AsyncEfficiencyCode",
    "ApprovalMode",
    "Sandbox",
    "ChatgptLoginHandle",
    "DeviceCodeLoginHandle",
    "AsyncChatgptLoginHandle",
    "AsyncDeviceCodeLoginHandle",
    "Thread",
    "AsyncThread",
    "TurnHandle",
    "AsyncTurnHandle",
    "TurnResult",
    "Input",
    "InputItem",
    "RunInput",
    "TextInput",
    "ImageInput",
    "LocalImageInput",
    "SkillInput",
    "MentionInput",
    "retry_on_overload",
    "EfficiencyCodeError",
    "TransportClosedError",
    "JsonRpcError",
    "EfficiencyCodeRpcError",
    "ParseError",
    "InvalidRequestError",
    "MethodNotFoundError",
    "InvalidParamsError",
    "InternalRpcError",
    "ServerBusyError",
    "RetryLimitExceededError",
    "is_retryable_error",
]

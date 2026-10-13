from importlib.metadata import PackageNotFoundError, version

from .rewriter import prompt_rewriter as prompt_rewriter


def get_version() -> str:
    try:
        return version("my_ai_lib")
    except PackageNotFoundError:
        return "0.1.0-dev"


__version__ = get_version()

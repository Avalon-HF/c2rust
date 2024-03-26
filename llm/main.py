from fastapi import FastAPI

from code_interpreter import CodeInterpreter
from models import Message, Role
import web


app = FastAPI()
app.include_router(web.router)


if __name__ == '__main__':
    llm = CodeInterpreter()
    messages = [
        Message(role=Role.System, content="you are a lanuage expert"),
        Message(role=Role.User, content="hello"),
        Message(role=Role.Assistant, content="Hello! How can I assist you with your programming or computer science questions today?"),
        Message(role=Role.User, content="who are you?"),
    ]
    response = llm.generate(messages)

    print(response)

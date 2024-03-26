from fastapi import FastAPI

from code_interpreter import CodeInterpreter
from models import Message, Role
import web


app = FastAPI()
app.include_router(web.router)

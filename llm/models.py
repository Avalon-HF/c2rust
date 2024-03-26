from typing import Optional
from pydantic import BaseModel
from enum import Enum


class Role(Enum):
    System = 'system'
    Assistant = 'assistant'
    User = 'user'


class Message(BaseModel):
    """
    the message format for interacting with LLM
    """
    role: Role
    content: str

    def to_dict(self):
        return {
            'role': self.role.value,
            'content': self.content,
        }



class CompletionReq(BaseModel):
    role: str
    content: str

    def to_message(self) -> "Message":
        return Message(role=Role(self.role), content=self.content)


class CompletionResp(BaseModel):
    code: int
    msg: str
    data: Optional[str]

    @staticmethod
    def ok(data: str) -> "CompletionResp":
        return CompletionResp(code=200, msg="操作成功", data=data)
    
    @staticmethod
    def error(msg: str) -> "CompletionResp":
        return CompletionResp(code=500, msg=msg, data=None)

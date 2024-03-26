import traceback
from fastapi import APIRouter
from typing import List
from code_interpreter import CodeInterpreter
from models import CompletionReq, CompletionResp


router = APIRouter()

llm = CodeInterpreter()


@router.post("/completion")
async def completion(req: List[CompletionReq]) -> CompletionResp:
    messages = list(map(lambda x: x.to_message(), req))
    try:
        resp = llm.generate(messages)
        return CompletionResp.ok(resp)
    except:
        error = traceback.format_exc()    
        return CompletionResp.error(error)


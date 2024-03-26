import torch
from transformers import AutoTokenizer, AutoModelForCausalLM
from typing import List
from models import Message
import re
import sys


MODEL_PATH = "m-a-p/OpenCodeInterpreter-DS-6.7B"

RESPONSE_REG = re.compile(r'(?s)### Response:\s+(.*?)\s*<\|EOT\|>', re.MULTILINE)



class CodeInterpreter:
    def __init__(self):
        self.tokenizer = AutoTokenizer.from_pretrained(MODEL_PATH, padding_side="right", trust_remote_code=True, device_map={'':0})
        model = AutoModelForCausalLM.from_pretrained(MODEL_PATH, load_in_4bit=False, load_in_8bit=True, torch_dtype=torch.float16, device_map={'':0}, trust_remote_code=True)
        model.resize_token_embeddings(len(self.tokenizer))
        self.model = model.eval()


    def generate(self, messages: List[Message]) -> str:
        prompts = list(map(lambda x: x.to_dict(), messages))
        print(prompts)
        final_prompts = self.tokenizer.apply_chat_template(prompts, tokenize=False)
        inputs = self.tokenizer([final_prompts], return_tensors="pt")
        inputs = inputs.to('cuda')
        sample = self.model.generate(**inputs, max_length=4096)
        text = self.tokenizer.decode(sample[0])
        return extract_last_response(text)


def extract_last_response(text: str) -> str:
    texts = RESPONSE_REG.findall(text)
    if len(texts) > 0:
        return texts[-1]
    raise ValueError(f'no matched response block found, the original full text is: {text}')

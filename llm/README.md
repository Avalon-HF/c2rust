# OpenCodeInterpreter 大模型

[OpenCodeInterpreter](https://github.com/OpenCodeInterpreter/OpenCodeInterpreter): 一个致力于编程相关的开源大模型。


## 部署

1. 自行准备 Python3.9 运行环境或通过 conda 创建虚拟环境：
    
    ```bash
    conda create -n llm python=3.9
    ```
2. 激活虚拟环境并安装相关依赖：
    
    ```bash
    conda activate llm
    python3 -m pip install -r requirements.txt
    ```

3. 运行 web 服务（第一次运行会从 [huggingface](https://huggingface.co)下载模型文件 ≈ 13G）：
    ```bash
    bash run.sh
    ```
   
## 服务接口

### 1. 基本对话

> POST /completion

#### 请求体

```json
[
  {
    "role": "system",
    "content": "你是一个编程专家，专注于 C/C++ 源码至 Rust 源码的转换"
  },
  {
    "role": "user",
    "content": "ping"
  },
  {
    "role": "assistant",
    "content": "pong"
  }
]
```

#### 响应体

```json5
{
  "code": 200, // 200: 成功，500: 失败
  "msg": "操作成功", // 或错误信息
  "data": "大模型作为 assistant 角色回复的内容"
  
}
```
## Proxying Copilot API to OpenAI's GPT-4 API

### Usage

1. Start the Server
```bash
export GHU_TOKEN=ghu_xxxx; ./copilot2chat
# or 
GHU_TOKEN="ghu_xxx" nohup ./copilot2chat &
```

**Or**

```bash
sh start.sh start # start the server then input the GHU_TOKEN manually
sh start.sh stop # stop the server
sh start.sh restart # restart the server
```

**Using docker**
  
```bash
docker run --name copilot2chat -dit -p 2088:2088 -e GHU_TOKEN=ghu_xxx ynlbq/copilot2chat:latest
```

2. Server Usage

The server can be used in the same way as the OpenAI API, but the endpoint URL to use is `http://127.0.0.1:2088/v1/chat/completions`.

Example:

```bash
curl --location 'http://localhost:2088/v1/chat/completions' \
--header 'Content-Type: text/plain' \
--data '{
    "model": "gpt-4",
    "messages": [{
      "role": "user", "content": "write a proxy using rust language"
    }],
    "presence_penalty": 0,
    "stream": false,
    "temperature": 0.8,
    "top_p": 1
  }'
```

You can also use a third-party client, such as [ChatGPT-Web](https://github.com/Chanzhaoyu/chatgpt-web) or any other client.

Example:

```bash
docker run \
  --name chatgpt-web \
  -d \
  -p 3002:3002 \
  -e AUTH_SECRET_KEY=123456 \
  -e OPENAI_API_KEY='xxxx' \
  -e OPENAI_API_BASE_URL=http://127.0.0.1:2088 \
  -e OPENAI_API_MODEL=gpt-4 \
  chenzhaoyu94/chatgpt-web
```

### How to Obtain a GHU_TOKEN?

Visit this link [https://cocopilot.org/copilot/token](https://cocopilot.org/copilot/token), log in via GitHub, and then copy the token.
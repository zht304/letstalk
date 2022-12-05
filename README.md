

# 目标

- 保护隐私，数据只在各端存储，server只做中转，不做任何会话数据存储；
- 成员信任基于公钥的线下交换和认证；取决于端。
- 端到端数据加密。中间节点无法解密数据。

# 角色

talkerA  talkerB  server

## server

- 并发数量： 
- 存储：
   - talker信息：公钥、ID、昵称
   - **密码**（二期） 如果不设密码，开放式存在冒充登录的风险，因为公钥是放在服务器上。获得公钥后没密码就可登录，虽然会话内容不可解密，但加密的会话内容可得，有风险。
   - 普通安全等级的群组成员列表。（三期，非必要，可由客户端维护）

## talker

- 存储会话密钥
- 存储群组信息
- 可请求刷新会话密钥（三期）

```
{
   pubkey,
   keyfmt,
   uid,
   name,
   state,
   websocket,
}

hashmap[uid=SHA256(pubkey)]

```

## 群组管理员

- 会话密钥生成与分发
- 成员管理


# 过程

- ## 注册
   - 

- ## 登录
   - 密码验证 
   - 登录后同步最新的会话密钥 [ 群组信息 ]

- ## 建组
   - 普通安全等级模式： 随机产生并分发对称会话密钥，只在会话成员持有。会话密钥由各自公钥加密后传输。
      - 由server分发同一份加密数据。（共同的会话密钥）
      - 群组成员列表允许存储于服务器。
      - 会话数据不在服务器存储（解密风险），只能群发到在线成员。

   - 高安全等级模式：发给所有talker的数据都用自个的公钥加密。端解密会话内容的时间长。
      - 该模式下，群组信息维护于端的本地，服务器不做任何存储。数据等于单发给了所有在线成员。
      - 后期可做一定量的数据缓存（公钥加密数据），用户上线后同步。



# 协议

websocket(secure)  80/443端口大多不受限制   
talker -- ws(s) -- server  

- handshake
- ping/pong
- binary
- text


## 控制报文

   proto: **wss**. text masked.  
   payload: json

### register
- talker -> sever


```json
{
  "Request": {
    "request_id": "001",
    "payload": {
      "Register": {
        "id": "thomas",
        "name": "huoche",
        "pub_key": "ssh-rsa AAA",
        "password": "12345",
        "key_format": "SshKey"
      }
    }
  }
}
```

- server -> talker:

```json
{ 
   "Response": { 
      "request_id": "001", 
      "payload": { 
         "RegisterReply": { 
            "uid": "62b1ace7211de774b80196be0c8ccc40db9260c684ddaa9f22bd9fd62ba9576a" 
         } 
      } 
   } 
}
```

### common error response
- server -> talker:

```json
{ 
   "Response": { 
      "request_id": "001", 
      "payload": { 
         "Error": { 
            "reason": "some reason" 
         } 
      } 
   } 
}
```

### login

- talker -> server:

   proto:wss text masked  
   payload: json  
```json
{
  "Request": {
    "request_id": "001",
    "payload": {
      "Login": {
        "id": "thomas",
        "pub_key": "ssh-rsa AAA",
        "password": "12345",
        "key_format": "SshKey"
      }
    }
  }
}
```

- server -> talker

```json
{ 
   "Response": { 
      "request_id": "001", 
      "payload": { 
         "LoginReply": { 
            "uid": "62b1ace7211de774b80196be0c8ccc40db9260c684ddaa9f22bd9fd62ba9576a" 
         } 
      } 
   } 
}
```


### create-group

- talker -> server:

```json
```

- server -> talker:

```json
```
### list

- talker -> server: list(talkers)

```json
```

- server -> talk:   reply_list() -> talker {uid=SHA(pubkey), name}


## 会话数据:

proto: ws binary.

talkeA -> server transparently fwd -> talkerB

- talker -> server: msg(uid=SHA(pubkey))

Note: pubkey can be exchanged and verified offline..

FRAME:
bit
```
+0---+4---+8---+C---+
|    |    |    |    |
+0---+4---+8---+C---+
|    |    |    |    |
+0---+4---+8---+C---+
```

MSG:

databse:
users




# State
```bash
curl -X POST http://127.0.0.1:8080 -H "Content-Type: application/json" --data '{"jsonrpc": "2.0", "method": "kv_raft_state", "params": [], "id": 1}' 
```

## Set
```bash
curl -X POST http://127.0.0.1:8080 -H "Content-Type: application/json" --data '{"jsonrpc": "2.0", "method": "kv_set", "params": ["hola", "como estas"], "id": 1}'
```

## Get
```bash
curl -X POST http://127.0.0.1:8080 -H "Content-Type: application/json" --data '{"jsonrpc": "2.0", "method": "kv_get", "params": ["hola"], "id": 1}'
```

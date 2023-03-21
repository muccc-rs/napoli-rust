napoli-pain web frontend
===

yeeeeeeeew

running
---

Start the backend:

```
cargo build -p napoli-server
target/debug/napoli-server
```

Add an order:

```
grpcurl -format text -plaintext -d 'menu_url: "https://q3k.org/"' [::1]:50051 napoli.OrderService.CreateOrder
```

Start the webapp dev server:

```
cd napoli-pain/
trunk serve
```

Then point your browser to 127.0.0.1:8080.
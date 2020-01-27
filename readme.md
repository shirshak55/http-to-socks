# Http Proxy To Sock5 Tunnel

> Yes you can browse https using http proxy.

In firefox we can directly go to preference > Network settings and add the proxy

Client <-> Http Proxy <-> Socks Proxy <-> Target Website

Client connects to http proxy and connects to target website via socks proxy and after getting informations etc forwards it to client.

```shell
$ SERVER_SOADDR=0.0.0.0:8100 SOCKS_SOADDR=nyc.socks.ipvanish.com:1080 SOCKS_USERNAME=Ho6M3LOMmjcg SOCKS_PASSWORD=mnA5k4GDQKNI cargo run
```

Currently under development better optimized version will be released on future. So please don't use it in production or anything else.

Warning: Port below 1024 requires root permession at least on linux.

## Thanks:

- o0Ignition0o
- Bastion Teams

### Licence:

Dual Licence (Apache / Mit) like other crates

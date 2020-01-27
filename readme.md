# Http Proxy To Sock5 Tunnel

> Yes you can browse https using http proxy.

In firefox we can directly go to preference > Network settings and add the proxy

Client <-> Http Proxy <-> Socks Proxy <-> Target Website

Client connects to http proxy and connects to target website via socks proxy and after getting informations etc forwards it to client.

```shell
$ SERVER_SOADDR=0.0.0.0:8100 SOCKS_SOADDR=PROXY_ADDR_HERE SOCKS_USERNAME=USERNAMEHERE SOCKS_PASSWORD=PASSWORDHERE cargo run
```

Better Optimized version will be released on future. So please don't use it in production or anything else.

## Thanks:

o0Ignition0o
Bastion Teams

### Licence:

Dual Licence (Apache / Mit) like other crates

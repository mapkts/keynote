# HTTP协议

**HTTP**是⼀个专门在**两点**之间**传输**⽂字、图⽚、⾳频、视频等**超⽂本数据**的**约定和规范**。

## 常见的状态码

| 状态码 | 类别 | 描述 |
| --- | :-: | --- |
| 1xx | info | 提示信息，表示目前处于中间状态 |
| 200 OK | success | 表示一切正常，返回的响应头会有body数据(HEAD请求除外) |
| 204 No Content | success | 和200 OK基本相同，但响应头没有body数据 |
| 206 Partial Content | success | 用于分块下载和断点续传，表示返回的body数据只是一部分 |
| 301 Moved Permanently | redirect | 表示永久重定向，说明请求的资源不存在了，需要改用新的URL再次访问 |
| 302 Found | redirect | 表示临时重定向，说明请求的资源还在，但暂时需要用另一个URL访问 |
| 304 Not Modified | redirect | 表示缓存重定向，表示资源未修改，重定向已存在的缓冲⽂件，用于缓存控制 |
| 400 Bad Request | client error | 表示客户端请求的报⽂有错误，但只是个笼统的错误 |
| 403 Forbidden | client error | 表示服务器禁⽌访问资源，并不是客户端的请求出错 |
| 404 Not Found | client error | 表示请求的资源在服务器上不存在或未找到 |
| 500 Internal Server Error | server error | 和400类似，是一个笼统的错误码 |
| 501 Not Implemented | server error | 表示客户端请求的功能还不⽀持 |
| 502 Bad Gateway | server error | 网关/代理服务器返回的错误码，表示自身正常，但访问后端服务器发生了错误 |
| 503 Service Unavailable | server error | 表示服务器当前很忙，暂时⽆法响应服务器 |

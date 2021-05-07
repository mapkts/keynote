# systemd

systemd是一个Linux系统基础组件的集合，提供了一个系统和服务管理器，运行为PID 1并负责启动其它程序[^1]。systemd目前已成为大部分Linux发行版的标准配置，可用以下命令验证当前系统是否使用systemd：

```bash
# 第一行有systemd的话表示pid 1为systemd
ps -A | grep systemd
```

## Unit

systemd将系统资源统称为**Unit**，并划分为以下12种类型：

| UNIT TYPE | DESCRIPTION |
| --- | --- |
| service | 系统服务 |
| target | 多个unit构成的一个组 |
| device | 硬件设备 |
| mount | 文件系统的挂载点 |
| automount | 自动挂载点 |
| path | 文件路径 |
| scope | 不是由systemd启动的外部进程 |
| slice | 进程组 |
| snapshot | systemd快照 |
| socket | 用于进程间通信的socket |
| swap | swap文件 |
| timer | 定时器 |

使用`systemctl list-units`命令可查看正在运行的Unit（不带参数），每个Unit都以`.TYPE`结尾以标识其类型。常用的用法还有：

```bash
# 列出所有Unit
systemctl list-units --all
```

```bash
# 列出所有没有运行的Unit
systemctl list-units --all --state=inactive
```

```bash
# 列出所有加载失败的Unit
systemctl list-units --failed
```

```bash
# 列出正在运行的指定类型的Unit，这里指定类型为`service`
systemctl list-units --type=service
```

使用`systemctl status`命令可查看系统状态（不带参数）和指定Unit的状态。

```bash
# 查看sshd服务的状态
systemctl status sshd.service
```

使用systemctl命令时通常需要使用包括扩展名的单元全称（例如`sshd.service`），但在以下情形下可使用简写形式：

- 如果无扩展名，systemctl默认扩展名为`.service`，例如`sshd`和`sshd.service`是等价的。
- 挂载点会自动转化为相应的`.mount`单元，例如`/home`和`home.mount`是等价的。
- 设备会自动转化为相应的`.device`单元，例如`/dev/sda2`和`dev-sda2.device`是等价的。

## Unit管理

以下的命令用于控制一个Unit的运行状态，注意这些命令都需要sudo权限：

```bash
# 启动一个服务
sudo systemctl start redis.service

# 停止一个服务
sudo systemctl stop redis.service

# 重启一个服务
sudo systemctl restart redis.service

# 杀死一个服务的所有进程，适用于stop命令无响应的情形
sudo systemctl kill redis.service
```

## Unit配置文件

systemd默认从`/etc/systemd/system/`目录读取Unit配置文件，里面大部分为符号链接，真正的配置文件存放在`/usr/lib/systemd/system/`目录下。

可用`systemctl enable`和`systemctl disable`两个命令在上面两个目录之间创建、断开符号链接（直接用ln命令也Ok）：

```bash
# 创建符号链接
sudo systemctl enable redis.service

# 断开符号链接
sudo systemctl disable redis.service
```

`systemctl list-unit-files`命令可查看所有配置文件，`systemctl cat unit.service`命令可查看指定Unit的具体配置。

如果修改了配置文件，需要让systemd重新加载配置文件，然后重新启动，否则修改不会生效。

```bash
# 这里假设修改了redis服务的配置
sudo systemctl daemon-reload
sudo systemctl restart redis.service
```

> 详细的配置文档可从[这里](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)找到。

## 一个简单的echo服务

```text
[Unit]
Description=Testing-purposed echo service
After=network.target sshd.service
Wants=sshd.service
Requires=network.target

[Service]
ExecStartPre=-/bin/bash -c "echo start echo service"
ExecStartPre=-/bin/bash -c "echo start echoing"
ExecStart=/bin/bash -c "for (( i=1; i<=5; i++ )) ; do echo 42 && sleep 1s ; done"
ExecStartPost=-/bin/bash -c "echo before or after 42"
ExecStop=-/bin/bash -c "echo stop echo service"
ExecReload=/bin/bash -c "for (( i=1; i<=5; i++ )) ; do echo 42 && sleep 1s ; done"
Type=idle
Restart=always
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

这里我们通过以上一个简单的echo服务来说明常见配置项的用法。我们可以看到，Unit配置文件分为三个区块：

- `[Unit]`：定义不依赖于Unit类型的通用信息。
- `[Service]`：定义如何启动该服务，不是service类型的Unit无该区块。
- `[Install]`：定义如何安装这个配置文件，即怎样做到开机启动。

每个字段的含义如下，注意所有Unit配置文件中的所有字段都是case-sensitive的：

- `[Description]`：对当前服务的简单描述，这里定义的描述可通过`systemctl status unit`、`systemctl list-units`等命令看到。
- `[After]`：定义应该在哪些Unit之后启动，这里表示echo服务需要在`network.target`和`sshd.target`之后启动；相应的，有一个`Before`字段定义该Unit应该在哪些Unit之前启动。
- `[Wants]`：定义弱依赖关系，这里表示即使终端断开了，`echo`服务也会继续运行。
- `[Requires]`：定义强依赖关系，这里表示如果网络服务中断了，`echo`服务也必须退出。
- `[ExecStartPre]`：启动服务之前执行的命令；命令之前的`-`表示抑制错误，即即使发生错误，也不会影响其他命令的执行。
- `[ExecStart]`：启动服务时执行的命令。作为`Service`块中最关键的命令，该字段必须设置。
- `[ExecStartPost]`：启动服务之后执行的命令。
- `[ExecReload]`：重启服务时执行的命令。
- `[ExecStop]`：停止服务时执行的命令。
- `[Type]`：定义启动类型，可以设置的值如下：
  - `simple`（默认值）：ExecStart字段启动的进程为主进程。
  - `forking`：ExecStart字段将以fork()方式启动，此时父进程将会退出，子进程将成为主进程。
  - `oneshot`：类似于simple，但只执行一次，systemd会等待它执行完，才启动其他服务。
  - `dbus`：类似于simple，但会等待收到D-Bus信号后启动。
  - `notify`：类似于simple，启动结束后会发出通知信号，然后systemd再启动其他服务。
  - `idle`：类似于simple，但是要等到其他任务都执行完，才会启动该服务。一种使用场合是为让该服务的输出，不与其他服务的输出相混合。
- `[Restart]`：定义systemd重启该服务的方式，可以设置的值如下：
  - `no`（默认值）：退出后不会重启。
  - `on-success`：只有正常退出时（退出状态码为0），才会重启。
  - `on-failure`：非正常退出时（退出状态码非0），包括被信号终止和超时，才会重启。
  - `on-abnormal`：只有被信号终止和超时，才会重启。
  - `on-abort`：只有在收到没有捕捉到的信号终止时，才会重启。
  - `on-watchdog`：超时退出，才会重启。
  - `always`：不管是什么退出原因，总是重启。
- `[RestartSec]`：表示systemd重启该服务之前，需要等待多少秒。
- `[WantedBy]`：表示该服务所在的Target，在运行`systemctl enable echo.service`时，`echo.service`的符号链接会在`/etc/systemd/system/`下的`multi-user.target.wants`子目录中。常用的Target有两个：一个是`multi-user.target`，表示多用户命令行状态；另一个是`graphical.target`，表示图形用户状态，它依赖于`multi-user.target`。

接下来我们将`echo`服务部署到机器上，看看是否能正常运行。

首先，在`/usr/lib/systemd/system/`目录下创建一个`echo.service`文件：

```bash
sudo vi /usr/lib/systemd/system/echo.service
```

然后将上面的`echo.service`配置复制进去，保存退出后执行以下命令加载配置：

```bash
sudo systemctl enable echo
```

加载完成后，执行以下命令启动echo服务：

```bash
sudo systemctl start echo
```

然后使用`journalctl`命令查看实时的输出：

```bash
sudo journalctl -fu echo
```

测试完成后，用以下命令彻底清除测试用的echo服务：

```bash
sudo systemctl disable echo
sudo rm /usr/lib/systemd/system/echo.service
sudo systemctl daemon-reload
sudo systemctl reset-failed
```

### 参考资料

[^1]: [https://wiki.archlinux.org/title/systemd](httpssystemd://wiki.archlinux.org/title/)

[^2]: [http://www.ruanyifeng.com/blog/2016/03/systemd-tutorial-commands.html](http://www.ruanyifeng.com/blog/2016/03/systemd-tutorial-commands.html)

[^3]: [http://www.ruanyifeng.com/blog/2016/03/systemd-tutorial-part-two.html](http://www.ruanyifeng.com/blog/2016/03/systemd-tutorial-part-two.html)

[^4]: [https://www.freedesktop.org/software/systemd/man/systemd.unit.html](https://www.freedesktop.org/software/systemd/man/systemd.unit.html)

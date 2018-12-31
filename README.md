# coolq-app-python
使用python编写酷q应用

将[应用](https://github.com/juzi5201314/coolq-app-python/releases/)放入酷q加载，重启。会在应用目录生成py_script目录。  
将py应用放入，重启酷q即可加载。  
例如名为test的应用：CoolqAir\data\app\net.systir.coolq_app_python\py_script\test

## Example
请看[Example](https://github.com/juzi5201314/coolq-app-python/tree/master/example/test)  
使用coolq_sdk.register(事件名字, 函数)来注册事件监听器。事件名字与参数[请看此处](https://github.com/juzi5201314/coolq-sdk-rust/blob/master/src/lib.rs#L86-L192)。  
调用酷q api使用coolq_sdk.apiname(args...)。如coolq_sdk.send_private_msg(qq号码, message)。[全部api列表](https://github.com/juzi5201314/coolq-sdk-rust/blob/master/src/lib.rs#L194-L367)

api与事件与[rust sdk](https://github.com/juzi5201314/coolq-sdk-rust)一致。

## 环境
本插件运行需要python3.5+，如在windows上运行，只需要安装python环境即可。pip同理。  
如在docker中运行，请容后解决（我还没想到，跑）。  
或者手动在wine里安装python，但是从py3.5开始，官网就没有提供.msi了。

## 错误处理
因为pyo3库的bug，coolq-app-python暂时无法捕获python的异常。如果python代码出现错误，会导致酷q崩溃。待解决。

## 其他功能
慢慢来，别急。
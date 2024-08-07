## perseus-py

对外提供的python包， 用于操作整个仿真世界



### 开发

```shell
# 安装依赖
pip3 install virtualenv
pip3 install maturin==0.8.2

# 工程初始化
maturin init  
# python 虚拟环境创建
virtualenv test_env
# 启用
source py_env/bin/activate
# coding

# 编译
maturin develop
```



### 发布

```shell
maturin build
```


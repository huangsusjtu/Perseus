## 后端模块

- libcommon

   整个项目公共的数据结构

- libformat

   读写 场景文件的， 要支持 sdf 和 mjcf格式

- libphysic

   物理引擎的适配

- libsensor

   传感器仿真， 比如 imu，camera

- libsimulator   
  ​ 仿真主控逻辑

### 统计代码行数

```shell
 find .  -path "./target" -prune -o   -type f -name "*.rs"  | xargs wc -l
```



### 消除warning

```shell
cargo fix --workspace --all-targets
```


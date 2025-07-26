# glotus

这是一个自己在学习 opengl 时练手用的封装库，简单地封装了一些 opengl 的功能。

## TODO

- 输入状态机，支持多键同时输入。

## utils

```shell
# 工作区中新建项目
cargo init --lib project_lib
cargo init --bin project_bin

# 运行项目
cargo run -p <project_name>
```

## shader uniforms

以下是一些 shader 中不用声明就可以使用的全局变量，需要使用`#include "glotus.glsl"`导入即可。

```glsl
uniform mat3 normal_matrix; // 法线从模型空间到世界空间的矩阵
uniform mat4 model_matrix; // 点从模型空间到世界空间的矩阵
uniform mat4 view_matrix;   // 点从世界空间到摄像机空间的矩阵
uniform mat4 projection_matrix; // 点从摄像机空间到透视空间的矩阵

uniform vec3 light_position;  // 光的位置
uniform vec3 view_position; // 视角（摄像机）的位置
uniform vec4 light_color;  // 光的颜色
```

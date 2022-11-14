import { getWebGLContext, initShaders } from "@/core/lib/cuon-utils";

export class Contour {
    constructor(canvas) {
        this.gl = getWebGLContext(canvas);
    }
    draw(points){
        // 顶点着色程序
        const VSHADER_SOURCE =
            "attribute vec4 a_Position;" +
            "void main(){" +
            "   gl_Position = a_Position;" +
            "   gl_PointSize = 1.0;" +
            "}";
        // 片源着色器程序
        const FSHADER_SOURCE =
            "precision mediump float;" +
            "void main(){" +
            "   gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);" + // 设置颜色
            "}";
        const gl = this.gl;

        // 初始化着色器
        if(!initShaders(gl, VSHADER_SOURCE, FSHADER_SOURCE)){
            console.log('初始化着色器失败');
            return;
        }

        // （1）创建缓冲区对象
        const pointsBuffer = gl.createBuffer();
        if(!pointsBuffer){
            console.error("创建缓冲区对象失败");
            return;
        }
        // （2）把缓冲区对象绑定到目标
        gl.bindBuffer(gl.ARRAY_BUFFER, pointsBuffer);
        // （3）向缓冲区对象写入数据
        gl.bufferData(gl.ARRAY_BUFFER, points, gl.STATIC_DRAW);

        // 读取坐标信息
        const a_Position = gl.getAttribLocation(gl.program, "a_Position");
        // （4）把缓冲区对象分配给a_Position变量
        gl.vertexAttribPointer(a_Position, 2, gl.FLOAT, false, 0, 0);
        // （5）链接a_Position变量与分配给他的缓冲区对象
        gl.enableVertexAttribArray(a_Position);

        // 清空canvas
        gl.clearColor(0, 0, 0, 1);
        gl.clear(gl.COLOR_BUFFER_BIT);

        // 绘制三个点，不能超过缓冲区的点数
        // gl.drawArrays(gl.POINTS, 0, 3);
        // 绘制一个渐变三角形
        gl.drawArrays(gl.POINTS, 0, points.length / 2);
        gl.drawArrays(gl.LINE_LOOP, 0, points.length / 2);
    }
}
import { parseStreets } from "./lib/parseStreets";
import shader from "./shader.wgsl" with { type: "text" };

function throwError(message: string): never {
  alert(message);
  throw new Error(message);
}

// https://sheeptester.github.io/words-go-here/misc/blur.html
const format = navigator.gpu.getPreferredCanvasFormat();
const adapter = await navigator.gpu.requestAdapter();
if (!adapter) throwError("missing adapter");
const device = await adapter.requestDevice();
device.lost.then(({ reason, message }) => {
  throwError(`WebGPU device lost: ${message} (reason: ${reason})`);
});
device.pushErrorScope("internal");
device.pushErrorScope("out-of-memory");
device.pushErrorScope("validation");

const canvas = document.getElementById("canvas");
if (!(canvas instanceof HTMLCanvasElement)) throwError("missing canvas");
const context = canvas.getContext("webgpu");
if (!context) throwError("missing context");
context.configure({ device, format });

const module = device.createShaderModule({ code: shader });
const { messages } = await module.getCompilationInfo();
if (messages.some((message) => message.type === "error")) {
  console.log(messages);
  throwError(
    "Shader failed to compile:" +
      messages
        .map(
          (message) =>
            `\nblur.wgsl:${message.lineNum}:${message.linePos}: ${message.message}`,
        )
        .join(""),
  );
}
const pipeline = device.createRenderPipeline({
  layout: "auto",
  vertex: { module, entryPoint: "vertex_main" },
  fragment: {
    module,
    entryPoint: "fragment_main",
    targets: [{ format }],
  },
});

const validationError = await device.popErrorScope();
const memoryError = await device.popErrorScope();
const internalError = await device.popErrorScope();
if (validationError) {
  throw new TypeError(`WebGPU validation error: ${validationError.message}`);
}
if (memoryError) {
  throw new TypeError(`WebGPU out of memory error: ${memoryError.message}`);
}
if (internalError) {
  throw new TypeError(`WebGPU internal error: ${internalError.message}`);
}

const encoder = device.createCommandEncoder();
const pass = encoder.beginRenderPass({
  colorAttachments: [
    {
      view: context.getCurrentTexture().createView(),
      clearValue: [0.02, 0.04, 0.08, 1],
      loadOp: "clear",
      storeOp: "store",
    },
  ],
});
pass.setPipeline(pipeline);
pass.draw(3);
pass.end();
device.queue.submit([encoder.finish()]);

const streetsData = await fetch("./streets.bin")
  .then((r) => r.arrayBuffer())
  .then(parseStreets);
console.log(streetsData);

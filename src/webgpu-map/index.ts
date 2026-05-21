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

const streetsData = await fetch("./streets.bin")
  .then((r) => r.arrayBuffer())
  .then(parseStreets);
console.log(streetsData);

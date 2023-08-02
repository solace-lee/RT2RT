export class Material {
  // texture: GPUTextture
  // view: GPUTexttureView
  // sampler: GPUSampler
  // bindGroup: GPUBindGroup

  constructor() { }

  async initialize(device, url, bindGroupLayout) {
    const response = await fetch(url);
    const blob = await response.blob();
    const imageData = await createImageBitmap(blob);
    await this.loadImageBitmap(device, imageData);

    const viewDescriptor = {
      format: "rgba8unorm",
      dimension: "2d",
      aspect: "all",
      baseMipLevel: 0,
      mipLevelCount: 1,
      baseArrayLayer: 0,
      arrayLayerCount: 1,
    };

    this.view = this.texture.createView(viewDescriptor);

    const samplerDescriptor = {
      addressModeU: "repeat",
      addressModeV: "repeat",
      magFilter: "linear",
      minFilter: "nearest",
      mipmapFilter: "nearest",
      maxAnisotropy: 1,
    };

    this.sampler = await device.createSampler(samplerDescriptor);

    this.bindGroup = device.createBindGroup({
      layout: bindGroupLayout, // 绑定组布局
      entries: [
        {
          binding: 0, // 绑定点
          resource: this.view,
        },
        {
          binding: 1, // 绑定点
          resource: this.sampler,
        },
      ],
    });
  }

  async loadImageBitmap(device, imageData) {
    const textureDescriptor = {
      size: {
        width: imageData.width,
        height: imageData.height,
      },
      format: "rgba8unorm",
      usage:
        GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.COPY_DST |
        GPUTextureUsage.RENDER_ATTACHMENT,
    };

    this.texture = device.createTexture(textureDescriptor);

    device.queue.copyExternalImageToTexture(
      {
        source: imageData,
      },
      {
        texture: this.texture,
      },
      textureDescriptor.size
    );
  }
}

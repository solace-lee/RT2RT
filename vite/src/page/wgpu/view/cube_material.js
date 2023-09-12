export class CubeMapMaterial {
  // texture: GPUTexture
  // view: GPUTextureView
  // sampler: GPUSampler

  async initialize(device, urls) {
    const imageData = new Array(6);

    for (let i = 0; i < 6; i++) {
      const response = await fetch(urls[i]);
      const blob = await response.blob();
      imageData[i] = await createImageBitmap(blob);
    }
    await this.loadImageBitmaps(device, imageData);

    const viewDescriptor = {
      format: "rgba8unorm",
      dimension: "cube",
      aspect: "all",
      baseMipLevel: 0,
      mipLevelCount: 1,
      baseArrayLayer: 0,
      arrayLayerCount: 6,
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
    this.sampler = device.createSampler(samplerDescriptor);
  }

  async loadImageBitmaps(device, imageData) {
    const textureDescriptor = {
      dimension: "2d",
      size: {
        width: imageData[0].width,
        height: imageData[0].height,
        depthOrArrayLayers: 6,
      },
      format: "rgba8unorm",
      usage:
        GPUTextureUsage.TEXTURE_BINDING |
        GPUTextureUsage.COPY_DST |
        GPUTextureUsage.RENDER_ATTACHMENT,
    };

    this.texture = device.createTexture(textureDescriptor);

    for (let i = 0; i < 6; i++) {
      device.queue.copyExternalImageToTexture(
        { source: imageData[i] },
        { texture: this.texture, origin: [0, 0, i] },
        [imageData[i].width, imageData[i].height]
      );
    }
  }
}

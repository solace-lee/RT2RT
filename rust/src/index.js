import rtData from './RT.json' assert { type: 'json' }

// JS原生处理
function main() {
  const { column, row, layNum, rowPixelSpacing, columnPixelSpacing, thickness, data } = rtData

  // console.log(column, row, layNum, rowPixelSpacing, columnPixelSpacing, thickness, data);
  const minPx = [rowPixelSpacing, columnPixelSpacing, thickness].sort()
  console.log("寻找最小的像素间距", minPx[0])

  const { xSize, ySize, zSize } = calcBound(minPx[0], rowPixelSpacing, columnPixelSpacing, thickness, row, column, layNum)
  console.log(`volume大小为：X:${xSize}px, Y:${ySize}px, Z:${zSize}px`);
  const layerPx = zSize / layNum
  console.log(`一层图占用的像素数量：${layerPx}`);

  const physicalArray = makeToArray(data)
  const { data: pxData, bound } = physicalToPx(physicalArray, rowPixelSpacing, columnPixelSpacing)

  console.log('像素坐标及边界', bound);


}

main()

// 物理坐标转像素坐标
function physicalToPx(data, rowPixelSpacing, columnPixelSpacing) {
  let bound = { xMin: 0, xMax: 0, yMin: 0, yMax: 0 } // 边界值
  data.forEach((value) => {
    value.forEach(coords => {
      coords.forEach(coord => {
        coord.x = Math.ceil(coord.x / rowPixelSpacing)
        coord.y = Math.ceil(coord.y / columnPixelSpacing)
        if (coord.x < bound.xMin) {
          bound.xMin = coord.x
        } else if (coord.x > bound.xMax) {
          bound.xMax = coord.x
        }

        if (coord.y < bound.yMin) {
          bound.yMin = coord.y
        } else if (coord.y > bound.yMax) {
          bound.yMax = coord.y
        }
      })
    })
  })
  return { data, bound }
}

// 格式化成数组
function makeToArray(physicalData = {}) {
  const data = new Map()
  Object.entries(physicalData).forEach(([key, value]) => {
    if (value && value.edgeCoords) {
      data.set(Number(key), value.edgeCoords)
    }
  })
  return data
}

// 计算volume大小
function calcBound(minPx, rowPixelSpacing, columnPixelSpacing, thickness, row, column, layNum) {
  // 向上取整
  const xScale = Math.ceil(rowPixelSpacing / minPx * row)
  const yScale = Math.ceil(columnPixelSpacing / minPx * column)
  const zScale = Math.ceil(thickness * layNum / minPx)
  return { xSize: xScale, ySize: yScale, zSize: zScale }
}
